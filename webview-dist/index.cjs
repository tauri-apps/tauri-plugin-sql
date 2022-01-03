var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __markAsModule = (target) => __defProp(target, "__esModule", { value: true });
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __reExport = (target, module2, copyDefault, desc) => {
  if (module2 && typeof module2 === "object" || typeof module2 === "function") {
    for (let key of __getOwnPropNames(module2))
      if (!__hasOwnProp.call(target, key) && (copyDefault || key !== "default"))
        __defProp(target, key, { get: () => module2[key], enumerable: !(desc = __getOwnPropDesc(module2, key)) || desc.enumerable });
  }
  return target;
};
var __toCommonJS = /* @__PURE__ */ ((cache) => {
  return (module2, temp) => {
    return cache && cache.get(module2) || (temp = __reExport(__markAsModule({}), module2, 1), cache && cache.set(module2, temp), temp);
  };
})(typeof WeakMap !== "undefined" ? /* @__PURE__ */ new WeakMap() : 0);

// webview-src/index.ts
var webview_src_exports = {};
__export(webview_src_exports, {
  default: () => Database
});
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
module.exports = __toCommonJS(webview_src_exports);
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {});
//# sourceMappingURL=index.cjs.map