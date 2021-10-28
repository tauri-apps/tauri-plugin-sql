import { invoke } from '@tauri-apps/api/tauri';

class Database {
    constructor(path) {
        this.path = path;
    }
    static load(path) {
        return invoke('plugin:sql|load', {
            db: path
        }).then(() => new Database(path));
    }
    static get(path) {
        return new Database(path);
    }
    execute(query, bindValues) {
        return invoke('plugin:sql|execute', {
            db: this.path,
            query,
            values: bindValues !== null && bindValues !== void 0 ? bindValues : []
        }).then(([rowsAffected, lastInsertId]) => ({ rowsAffected, lastInsertId }));
    }
    select(query, bindValues) {
        return invoke('plugin:sql|select', {
            db: this.path,
            query,
            values: bindValues !== null && bindValues !== void 0 ? bindValues : []
        });
    }
}

export { Database as default };
