use futures::future::BoxFuture;
use serde::{ser::Serializer, Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::{
    error::BoxDynError,
    migrate::{
        MigrateDatabase, Migration as SqlxMigration, MigrationSource, MigrationType, Migrator,
    },
    Column, Pool, Row, TypeInfo,
};
use tauri::{
    command,
    plugin::{Plugin, Result as PluginResult},
    AppHandle, Invoke, Manager, Runtime, State,
};
use tokio::sync::Mutex;

use std::collections::HashMap;

#[cfg(feature = "sqlite")]
type Db = sqlx::sqlite::Sqlite;
#[cfg(feature = "mysql")]
type Db = sqlx::mysql::MySql;
#[cfg(feature = "postgres")]
type Db = sqlx::postgres::Postgres;

#[cfg(feature = "sqlite")]
type LastInsertId = i64;
#[cfg(not(feature = "sqlite"))]
type LastInsertId = u64;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sql(#[from] sqlx::Error),
    #[error(transparent)]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("database {0} not loaded")]
    DatabaseNotLoaded(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
struct DbInstances(Mutex<HashMap<String, Pool<Db>>>);

struct Migrations(Mutex<HashMap<String, MigrationList>>);

#[derive(Default, Deserialize)]
struct PluginConfig {
    #[serde(default)]
    preload: Vec<String>,
}

#[derive(Debug)]
pub enum MigrationKind {
    Up,
    Down,
}

impl From<MigrationKind> for MigrationType {
    fn from(kind: MigrationKind) -> Self {
        match kind {
            MigrationKind::Up => Self::ReversibleUp,
            MigrationKind::Down => Self::ReversibleDown,
        }
    }
}

/// A migration definition.
#[derive(Debug)]
pub struct Migration {
    pub version: i64,
    pub description: &'static str,
    pub sql: &'static str,
    pub kind: MigrationKind,
}

#[derive(Debug)]
struct MigrationList(Vec<Migration>);

impl MigrationSource<'static> for MigrationList {
    fn resolve(self) -> BoxFuture<'static, std::result::Result<Vec<SqlxMigration>, BoxDynError>> {
        Box::pin(async move {
            let mut migrations = Vec::new();
            for migration in self.0 {
                if matches!(migration.kind, MigrationKind::Up) {
                    migrations.push(SqlxMigration::new(
                        migration.version,
                        migration.description.into(),
                        migration.kind.into(),
                        migration.sql.into(),
                    ));
                }
            }
            Ok(migrations)
        })
    }
}

#[command]
async fn load(
    db_instances: State<'_, DbInstances>,
    migrations: State<'_, Migrations>,
    db: String,
) -> Result<()> {
    if !Db::database_exists(&db).await.unwrap_or(false) {
        Db::create_database(&db).await?;
    }
    let pool = Pool::connect(&db).await?;
    if let Some(migrations) = migrations.0.lock().await.remove(&db) {
        let migrator = Migrator::new(migrations).await?;
        migrator.run(&pool).await?;
    }
    db_instances.0.lock().await.insert(db.clone(), pool);
    Ok(())
}

#[command]
async fn execute(
    db_instances: State<'_, DbInstances>,
    db: String,
    query: String,
    values: Vec<JsonValue>,
) -> Result<(u64, LastInsertId)> {
    let mut instances = db_instances.0.lock().await;
    let db = instances.get_mut(&db).ok_or(Error::DatabaseNotLoaded(db))?;
    let mut query = sqlx::query(&query);
    for value in values {
        query = query.bind(value);
    }
    let result = query.execute(&*db).await?;
    #[cfg(feature = "sqlite")]
    let r = Ok((result.rows_affected(), result.last_insert_rowid()));
    #[cfg(feature = "mysql")]
    let r = Ok((result.rows_affected(), result.last_insert_id()));
    #[cfg(feature = "postgres")]
    let r = Ok((result.rows_affected(), 0));
    r
}

#[command]
async fn select(
    db_instances: State<'_, DbInstances>,
    db: String,
    query: String,
    values: Vec<JsonValue>,
) -> Result<Vec<HashMap<String, JsonValue>>> {
    let mut instances = db_instances.0.lock().await;
    let db = instances.get_mut(&db).ok_or(Error::DatabaseNotLoaded(db))?;
    let mut query = sqlx::query(&query);
    for value in values {
        query = query.bind(value);
    }
    let rows = query.fetch_all(&*db).await?;
    let mut values = Vec::new();
    for row in rows {
        let mut value = HashMap::default();
        for (i, column) in row.columns().iter().enumerate() {
            let info = column.type_info();
            let v = if info.is_null() {
                JsonValue::Null
            } else {
                match info.name() {
                    "VARCHAR" | "STRING" | "TEXT" => JsonValue::String(row.get(i)),
                    "BOOL" | "BOOLEAN" => {
                        if let Ok(b) = row.try_get(i) {
                            JsonValue::Bool(b)
                        } else {
                            let x: String = row.get(i);
                            JsonValue::Bool(x.to_lowercase() == "true")
                        }
                    }
                    "INT" | "NUMBER" | "INTEGER" | "BIGINT" | "INT8" => {
                        JsonValue::Number(row.get::<u32, usize>(i).into())
                    }
                    // "JSON" => JsonValue::Object(row.get(i)),
                    "BLOB" => JsonValue::Array(
                        row.get::<Vec<u8>, usize>(i)
                            .into_iter()
                            .map(|i| JsonValue::Number(i.into()))
                            .collect(),
                    ),
                    _ => JsonValue::Null,
                }
            };
            value.insert(column.name().to_string(), v);
        }
        values.push(value);
    }
    Ok(values)
}

/// Tauri SQL plugin.
pub struct TauriSql<R: Runtime> {
    migrations: Option<HashMap<String, MigrationList>>,
    invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> Default for TauriSql<R> {
    fn default() -> Self {
        Self {
            migrations: Some(Default::default()),
            invoke_handler: Box::new(tauri::generate_handler![load, execute, select]),
        }
    }
}

impl<R: Runtime> TauriSql<R> {
    /// Add migrations to a database.
    pub fn add_migrations(mut self, db_url: &str, migrations: Vec<Migration>) -> Self {
        self.migrations
            .as_mut()
            .unwrap()
            .insert(db_url.to_string(), MigrationList(migrations));
        self
    }
}

impl<R: Runtime> Plugin<R> for TauriSql<R> {
    fn name(&self) -> &'static str {
        "sql"
    }

    fn initialize(&mut self, app: &AppHandle<R>, config: serde_json::Value) -> PluginResult<()> {
        tauri::async_runtime::block_on(async move {
            let config: PluginConfig = if config.is_null() {
                Default::default()
            } else {
                serde_json::from_value(config)?
            };
            let instances = DbInstances::default();
            let mut lock = instances.0.lock().await;
            for db_url in config.preload {
                if !Db::database_exists(&db_url).await.unwrap_or(false) {
                    Db::create_database(&db_url).await?;
                }
                let pool = Pool::connect(&db_url).await?;
                if let Some(migrations) = self.migrations.as_mut().unwrap().remove(&db_url) {
                    let migrator = Migrator::new(migrations).await?;
                    migrator.run(&pool).await?;
                }
                lock.insert(db_url.clone(), pool);
            }
            drop(lock);
            app.manage(instances);
            app.manage(Migrations(Mutex::new(self.migrations.take().unwrap())));
            Ok(())
        })
    }

    fn extend_api(&mut self, message: Invoke<R>) {
        (self.invoke_handler)(message)
    }
}
