#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu,  Submenu};
use tauri_plugin_sql::{Migration, MigrationKind, TauriSql};

fn main() {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let submenu = Submenu::new("File", Menu::new().add_item(quit));
  let menu = Menu::new().add_submenu(submenu);

  tauri::Builder::default()
    .menu(menu)
    .on_menu_event(|event| match event.menu_item_id() {
      "quit" => {
        std::process::exit(0);
      }

      _ => {}
    })
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
