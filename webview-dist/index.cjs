"use strict";
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// webview-src/index.ts
var webview_src_exports = {};
__export(webview_src_exports, {
  default: () => Database
});
module.exports = __toCommonJS(webview_src_exports);
var import_tauri = require("@tauri-apps/api/tauri");
var Database = class {
  constructor(connection) {
    this.path = connection;
  }
  static async load(connection) {
    return await (0, import_tauri.invoke)("plugin:sql|load", {
      db: connection
    }).then(() => new Database(connection));
  }
  static get(connection) {
    return new Database(connection);
  }
  async execute(sql, bindValues) {
    return await (0, import_tauri.invoke)("plugin:sql|execute", {
      db: this.path,
      sql,
      values: bindValues ?? []
    }).then(([rowsAffected, lastInsertId]) => ({ rowsAffected, lastInsertId }));
  }
  async select(sql, bindValues) {
    return await (0, import_tauri.invoke)("plugin:sql|select", {
      db: this.path,
      sql,
      values: bindValues ?? []
    });
  }
  async close() {
    return await (0, import_tauri.invoke)("plugin:sql|close", {
      db: this.path
    });
  }
};
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {});
