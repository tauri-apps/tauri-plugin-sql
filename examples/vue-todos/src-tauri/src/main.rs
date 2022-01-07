#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Menu, MenuItem, Submenu};
use tauri_plugin_sql::{Migration, MigrationKind, TauriSql};

fn main() {
  let submenu = Submenu::new("File", Menu::new().add_native_item(MenuItem::Quit));
  let menu = Menu::new().add_submenu(submenu);

  tauri::Builder::default()
    .menu(menu)
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
