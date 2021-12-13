// webview-src/index.ts
import { invoke } from "@tauri-apps/api/tauri";
var Database = class {
  constructor(path) {
    this.path = path;
  }
  static async load(path) {
    return await invoke("plugin:sql|load", {
      db: path
    }).then((p) => new Database(p));
  }
  static get(path) {
    return new Database(path);
  }
  async execute(query, bindValues) {
    return await invoke("plugin:sql|execute", {
      db: this.path,
      query,
      values: bindValues != null ? bindValues : []
    }).then(([rowsAffected, lastInsertId]) => ({ rowsAffected, lastInsertId }));
  }
  async select(query, bindValues) {
    return await invoke("plugin:sql|select", {
      db: this.path,
      query,
      values: bindValues != null ? bindValues : []
    });
  }
};
export {
  Database as default
};
//# sourceMappingURL=index.js.map