#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri_plugin_sql::{Migration, MigrationKind, TauriSql};

fn main() {
  tauri::Builder::default()
    .plugin(TauriSql::default().add_migrations(
      "sqlite:test.db",
      vec![Migration {
        version: 1,
        description: "create todo",
        sql: include_str!("../migrations/1.sql"),
        kind: MigrationKind::Up,
      }],
    ))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
