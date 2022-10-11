export interface QueryResult {
    /** the number of rows affected by the query. */
    rowsAffected: number;
    /**
     * The last inserted `id`.
     *
     * This value is always `0` when using the Postgres driver. If the
     * last inserted id is required on Postgres, the `select` function
     * must be used, with a `RETURNING` clause
     * (`INSERT INTO todos (title) VALUES ($1) RETURNING id`).
     */
    lastInsertId: number;
}
/**
 * **Database**
 *
 * the database class serves as the primary interface for the frontend
 * to communicate to the backend's `tauri-plugin-sql` API.
 */
export default class Database {
    path: string;
    constructor(path: string);
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
    static load(path: string): Promise<Database>;
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
    static get(path: string): Database;
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
    execute(query: string, bindValues?: unknown[]): Promise<QueryResult>;
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
    select<T>(query: string, bindValues?: unknown[]): Promise<T>;
    /**
     * **close**
     *
     * Closes the database connection pool.
     *
     * @param db optionally state the name of a database if you are managing more than one; otherwise all database pools will be in scope
     */
    close(db?: string): Promise<boolean>;
}
