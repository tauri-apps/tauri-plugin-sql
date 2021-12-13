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
use tokio::{fs, sync::Mutex};

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

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

/// Separates a "connection string" into it's constituant parts
/// which are: (a) the database, and (b) the connection details.
///
/// In most cases you connect with network details but for a SQLite
/// database it is with a file based resource where the file based resource
/// will need to be combined with the app's designated directory in filesystem.
enum Connection {
    Sqlite(String, Option<PathBuf>),
    Mysql(String),
    Postgres(String),
}

impl Connection {
    fn add_app_directory(self, app_dir: &PathBuf) -> Self {
        match self {
            Connection::Sqlite(d, _) => Connection::Sqlite(d.clone(), Some(app_dir.clone())),
            _ => self,
        }
    }
}

impl From<&str> for Connection {
    fn from(connect: &str) -> Self {
        let (db_type, details) = connect
            .split_once(":")
            .expect("Couldn't parse the connection string for the database! Please ensure that your connection string has a ':' character in it.");
        let db_type = String::from(db_type).to_lowercase();

        match db_type.as_str() {
            "sqlite" => Connection::Sqlite(details.to_string(), None),
            "mysql" => Connection::Mysql(details.to_string()),
            "postgres" => Connection::Postgres(details.to_string()),
            _ => panic!("The connection string passed in has an unrecognized database and can not be used: {}", details)
        }
    }
}
impl From<MigrationKind> for Connection {
    fn from(connect: MigrationKind) -> Self {
        Connection::from(connect);
    }
}

impl From<String> for Connection {
    fn from(connect: String) -> Self {
        let s = connect.as_str();
        Connection::from(s)
    }
}

impl ToString for Connection {
    fn to_string(&self) -> String {
        match &self {
            Connection::Sqlite(file, Some(app_dir)) => String::from(
                ["sqlite:", app_dir.join(&file).to_str().unwrap_or(&file)]
                    .concat()
                    .to_string(),
            ),
            Connection::Sqlite(file, None) => String::from(["sqlite:", &file].concat().to_string()),
            Connection::Mysql(file) => String::from(["mysql:", &file].concat().to_string()),
            Connection::Postgres(file) => String::from(["postgres:", &file].concat().to_string()),
        }
    }
}

#[derive(Default)]
struct DbInstances(Mutex<HashMap<String, Pool<Db>>>);

/// Provides important contextual path locations for the runtime environment.
struct Locations {
    /// The "app path" is important when using Sqlite database as it
    /// establishes the full path to the database file.
    app: PathBuf,
    resources: PathBuf,
}

impl Locations {
    fn new<R: Runtime>(app: &AppHandle<R>) -> Locations {
        let app_dir = app
            .path_resolver()
            .app_dir()
            .expect("failed to parse app directory!");
        let resource_dir = app
            .path_resolver()
            .resource_dir()
            .expect("failed to parse resource directory!");

        Locations {
            app: app_dir.clone(),
            resources: resource_dir.clone(),
        }
    }
}

struct Migrations(Mutex<HashMap<String, MigrationList>>);

#[derive(Default, Deserialize)]

/// The PluginConfig allows for _pre-loading_ database connections.
/// This is distinct from the more typical arrangement where the
/// frontend webview would pass in a database connection string from
/// `load` command
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
    locations: State<'_, Locations>,
    migrations: State<'_, Migrations>,
    db: String,
) -> Result<String> {
    let conn = Connection::from(db)
        .add_app_directory(&locations.app)
        .to_string();

    let db_exists = Db::database_exists(&conn).await.unwrap_or(false);
    let app_dir_exists = Path::new(&locations.app).is_dir();

    if !app_dir_exists {
        match fs::create_dir(&locations.app).await {
            Ok(_) => (),
            Err(e) => panic!(
                "Unable to create the app directory [ {} ] for TODO app: {}",
                &locations.app.to_str().unwrap_or(""),
                e
            ),
        }
    }

    println!(
        "Connection string is: {}; DB exists {}; App dir exists: {}",
        &conn, db_exists, app_dir_exists
    );

    if !db_exists {
        Db::create_database(&conn).await?;
        println!("database was created");
    }

    println!("About to connect to pool");
    let pool = Pool::connect(&conn).await?;
    println!("Pool established");

    println!("Migrations: {:?}", migrations.0);

    if let Some(migrations) = migrations.0.lock().await.remove(&conn) {
        let migrator = Migrator::new(migrations).await?;
        migrator.run(&pool).await?;
    }
    println!("Locking instance established");

    db_instances.0.lock().await.insert(conn.clone(), pool);
    println!("instance locked");
    Ok(conn)
}

#[command]
async fn execute(
    locations: State<'_, Locations>,
    db_instances: State<'_, DbInstances>,
    db: String,
    query: String,
    values: Vec<JsonValue>,
) -> Result<(u64, LastInsertId)> {
    let mut instances = db_instances.0.lock().await;
    let conn = Connection::from(db)
        .add_app_directory(&locations.app)
        .to_string();
    let db = instances
        .get_mut(&conn)
        .ok_or(Error::DatabaseNotLoaded(conn))?;
    let mut query = sqlx::query(&query);
    for value in values {
        if value.is_string() {
            query = query.bind(value.as_str().unwrap().to_owned())
        } else {
            query = query.bind(value);
        }
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
    locations: State<'_, Locations>,
    db_instances: State<'_, DbInstances>,
    db: String,
    query: String,
    values: Vec<JsonValue>,
) -> Result<Vec<HashMap<String, JsonValue>>> {
    let mut instances = db_instances.0.lock().await;
    let conn = Connection::from(db)
        .add_app_directory(&locations.app)
        .to_string();
    let db = instances
        .get_mut(&conn)
        .ok_or(Error::DatabaseNotLoaded(conn))?;
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
                        JsonValue::Number(row.get::<i64, usize>(i).into())
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

            let locations = Locations::new(app);

            let instances = DbInstances::default();
            let mut lock = instances.0.lock().await;
            for db_url in config.preload {
                let conn = Connection::from(db_url)
                    .add_app_directory(&locations.app)
                    .to_string();

                if !Db::database_exists(&conn).await.unwrap_or(false) {
                    Db::create_database(&conn).await?;
                }
                let pool = Pool::connect(&conn).await?;

                if let Some(migrations) = self.migrations.as_mut().unwrap().remove(&conn) {
                    // TODO: iterate over migrations and replace the path to include the app_dir when using Sqlite

                    // let m = migrations.0.into_iter();
                    // let migrations = m.map(move |m| -> Migration {
                    //     let conn = Connection::from(m.kind)
                    //         .add_app_directory(&locations.app)
                    //         .to_string();

                    //     Migration {
                    //         kind: &conn.as_str(),
                    //         ..m
                    //     }
                    // });
                    let migrator = Migrator::new(migrations).await?;
                    migrator.run(&pool).await?;
                }
                lock.insert(conn.clone(), pool);
            }
            drop(lock);
            app.manage(locations);
            app.manage(instances);
            app.manage(Migrations(Mutex::new(self.migrations.take().unwrap())));
            Ok(())
        })
    }

    fn extend_api(&mut self, message: Invoke<R>) {
        (self.invoke_handler)(message)
    }
}
