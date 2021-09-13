use futures::future::BoxFuture;
use once_cell::sync::Lazy;
use serde::Deserialize;
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
    AppHandle, Invoke, Runtime,
};
use tokio::sync::Mutex;

use std::{collections::HashMap, sync::Arc};

#[cfg(feature = "sqlite")]
type Db = sqlx::sqlite::Sqlite;

fn db_instances() -> &'static Arc<Mutex<HashMap<String, Pool<Db>>>> {
    static DBS: Lazy<Arc<Mutex<HashMap<String, Pool<Db>>>>> = Lazy::new(Default::default);
    &DBS
}

#[derive(Deserialize)]
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
    fn resolve(self) -> BoxFuture<'static, Result<Vec<SqlxMigration>, BoxDynError>> {
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
async fn sqlx_execute(db: String, query: String) -> u64 {
    let mut instances = db_instances().lock().await;
    let db = instances.get_mut(&db).unwrap();
    sqlx::query(&query)
        .execute(&*db)
        .await
        .unwrap()
        .rows_affected()
}

#[command]
async fn sqlx_select(db: String, query: String) -> Result<Vec<HashMap<String, JsonValue>>, String> {
    let mut instances = db_instances().lock().await;
    let db = instances.get_mut(&db).unwrap();
    let rows = sqlx::query(&query)
        .fetch_all(&*db)
        .await
        .map_err(|e| e.to_string())?;
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
                    "BOOL" | "BOOLEAN" => JsonValue::Bool(row.get(i)),
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
    migrations: HashMap<String, MigrationList>,
    invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> Default for TauriSql<R> {
    fn default() -> Self {
        Self {
            migrations: Default::default(),
            invoke_handler: Box::new(tauri::generate_handler![sqlx_execute, sqlx_select]),
        }
    }
}

impl<R: Runtime> TauriSql<R> {
    /// Add migrations to a database.
    pub fn add_migrations(mut self, db_url: &str, migrations: Vec<Migration>) -> Self {
        self.migrations
            .insert(db_url.to_string(), MigrationList(migrations));
        self
    }
}

impl<R: Runtime> Plugin<R> for TauriSql<R> {
    fn name(&self) -> &'static str {
        "sql"
    }

    fn initialize(&mut self, _app: &AppHandle<R>, config: serde_json::Value) -> PluginResult<()> {
        tauri::async_runtime::block_on(async move {
            let config: PluginConfig = serde_json::from_value(config)?;
            let mut instances = db_instances().lock().await;
            for db_url in config.preload {
                if !Db::database_exists(&db_url).await.unwrap_or(false) {
                    Db::create_database(&db_url).await?;
                }
                let pool = Pool::connect(&db_url).await?;
                if let Some(migrations) = self.migrations.remove(&db_url) {
                    let migrator = Migrator::new(migrations).await.unwrap();
                    migrator.run(&pool).await.unwrap();
                }
                instances.insert(db_url.clone(), pool);
            }
            Ok(())
        })
    }

    fn extend_api(&mut self, message: Invoke<R>) {
        (self.invoke_handler)(message)
    }
}
