import { invoke } from '@tauri-apps/api/tauri';

/**
 * **Database**
 *
 * the database class serves as the primary interface for the frontend
 * to communicate to the backend's `tauri-plugin-sql` API.
 */
class Database {
    constructor(path) {
        this.path = path;
    }
    /**
     * **load**
     *
     * A static initializer which connects to the underlying database
     * and returns a `Database` instance once a connecion to the database
     * is established.
     *
     * # Sqlite
     *
     * The path is relative to `tauri::api::path::BaseDirectory::App` and must start with `sqlite:`.
     *
     * ```ts
     * const db = await Database.load("sqlite:test.db");
     * ```
     */
    static async load(path) {
        return await invoke('plugin:sql|load', {
            db: path
        }).then((p) => new Database(p));
    }
    /**
     * **get**
     *
     * A static initializer which synchronously returns an instance of
     * the Database class while deferring the actual database connection
     * until the first invokation or selection on the database.
     *
     * # Sqlite
     *
     * The path is relative to `tauri::api::path::BaseDirectory::App` and must start with `sqlite:`.
     *
     * ```ts
     * const db = Database.get("sqlite:test.db");
     * ```
     */
    static get(path) {
        return new Database(path);
    }
    /**
     * **execute**
     *
     * Passes a SQL expression to the database for execution.
     *
     * ```ts
     * const result = await db.execute(
     *    "UPDATE todos SET title = $1, completed = $2 WHERE id = $3",
     *    [ todos.title, todos.status, todos.id ]
     * );
     * ```
     */
    async execute(query, bindValues) {
        return await invoke('plugin:sql|execute', {
            db: this.path,
            query,
            values: bindValues !== null && bindValues !== void 0 ? bindValues : []
        }).then(([rowsAffected, lastInsertId]) => ({ rowsAffected, lastInsertId }));
    }
    /**
     * **select**
     *
     * Passes in a SELECT query to the database for execution.
     *
     * ```ts
     * const result = await db.select(
     *    "SELECT * from todos WHERE id = $1", id
     * );
     * ```
     */
    async select(query, bindValues) {
        return await invoke('plugin:sql|select', {
            db: this.path,
            query,
            values: bindValues !== null && bindValues !== void 0 ? bindValues : []
        });
    }
}

export { Database as default };
