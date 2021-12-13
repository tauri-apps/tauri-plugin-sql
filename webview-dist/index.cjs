var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __markAsModule = (target) => __defProp(target, "__esModule", { value: true });
var __export = (target, all) => {
  __markAsModule(target);
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __reExport = (target, module2, desc) => {
  if (module2 && typeof module2 === "object" || typeof module2 === "function") {
    for (let key of __getOwnPropNames(module2))
      if (!__hasOwnProp.call(target, key) && key !== "default")
        __defProp(target, key, { get: () => module2[key], enumerable: !(desc = __getOwnPropDesc(module2, key)) || desc.enumerable });
  }
  return target;
};
var __toModule = (module2) => {
  return __reExport(__markAsModule(__defProp(module2 != null ? __create(__getProtoOf(module2)) : {}, "default", module2 && module2.__esModule && "default" in module2 ? { get: () => module2.default, enumerable: true } : { value: module2, enumerable: true })), module2);
};

// webview-src/index.ts
__export(exports, {
  default: () => Database
});
var import_tauri = __toModule(require("@tauri-apps/api/tauri"));
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
      values: bindValues != null ? bindValues : []
    }).then(([rowsAffected, lastInsertId]) => ({ rowsAffected, lastInsertId }));
  }
  async select(query, bindValues) {
    return await (0, import_tauri.invoke)("plugin:sql|select", {
      db: this.path,
      query,
      values: bindValues != null ? bindValues : []
    });
  }
};
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {});
//# sourceMappingURL=index.cjs.map