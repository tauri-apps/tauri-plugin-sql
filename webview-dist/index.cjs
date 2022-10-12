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
  constructor(path) {
    this.path = path;
  }
  static async load(path) {
    return await (0, import_tauri.invoke)("plugin:sql|load", {
      db: path
    }).then((p) => new Database(p));
  }
  static get(path) {
    return new Database(path);
  }
  async execute(query, bindValues) {
    return await (0, import_tauri.invoke)("plugin:sql|execute", {
      db: this.path,
      query,
      values: bindValues ?? []
    }).then(([rowsAffected, lastInsertId]) => ({ rowsAffected, lastInsertId }));
  }
  async select(query, bindValues) {
    return await (0, import_tauri.invoke)("plugin:sql|select", {
      db: this.path,
      query,
      values: bindValues ?? []
    });
  }
  async select_one(query, bindValues) {
    return await (0, import_tauri.invoke)("plugin:sql|select_one", {
      db: this.path,
      query,
      values: bindValues ?? []
    });
  }
  async close(db) {
    return await (0, import_tauri.invoke)("plugin:sql|close", {
      db
    });
  }
};
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {});
