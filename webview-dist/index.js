// webview-src/index.ts
import { invoke } from "@tauri-apps/api/tauri";
var Database = class {
  constructor(connection) {
    this.path = connection;
  }
  static async load(connection) {
    return await invoke("plugin:sql|load", {
      db: connection
    }).then(() => new Database(connection));
  }
  static get(connection) {
    return new Database(connection);
  }
  async execute(sql, bindValues) {
    return await invoke("plugin:sql|execute", {
      db: this.path,
      sql,
      values: bindValues ?? []
    }).then(([rowsAffected, lastInsertId]) => ({ rowsAffected, lastInsertId }));
  }
  async select(sql, bindValues) {
    return await invoke("plugin:sql|select", {
      db: this.path,
      sql,
      values: bindValues ?? []
    });
  }
  async close() {
    return await invoke("plugin:sql|close", {
      db: this.path
    });
  }
};
export {
  Database as default
};
