import { invoke } from "@tauri-apps/api/tauri";

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

export type DbConnection = `${`sqlite` | `postgres` | `mysql`}:${string}`;

/**
 * **Database**
 *
 * the database class serves as the primary interface for the frontend
 * to communicate to the backend's `tauri-plugin-sql` API.
 *
 * @connection  is a DB connection string like `sqlite:test.db`, etc.
 */
export default class Database {
  path: DbConnection;
  constructor(connection: DbConnection) {
    this.path = connection;
  }

  /**
   * **load**
   *
   * A static initializer which connects to the underlying database
   * and returns a `Database` instance once a connection to the database
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
  static async load<C extends DbConnection>(connection: C): Promise<Database> {
    return await invoke<string>("plugin:sql|load", {
      db: connection
    }).then(() => new Database(connection));
  }

  /**
   * **get**
   *
   * A static initializer which synchronously returns an instance of
   * the Database class while deferring the actual database connection
   * until the first invocation or selection on the database.
   *
   * # Sqlite
   *
   * The path is relative to `tauri::api::path::BaseDirectory::App` and must start with `sqlite:`.
   *
   * ```ts
   * const db = Database.get("sqlite:test.db");
   * ```
   */
  static get(connection: DbConnection): Database {
    return new Database(connection);
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
  async execute(sql: string, bindValues?: unknown[]): Promise<QueryResult> {
    return await invoke<[number, number]>("plugin:sql|execute", {
      db: this.path,
      sql,
      values: bindValues ?? []
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
  async select<T = unknown[]>(sql: string, bindValues?: unknown[]): Promise<T> {
    return await invoke("plugin:sql|select", {
      db: this.path,
      sql,
      values: bindValues ?? []
    });
  }

  /**
   * **close**
   *
   * Closes the database connection pool explicitly (note: all DBs are closed
   * automatically on application exit).
   *
   * @param db optionally state the name of a database if you are managing more than one; otherwise all database pools will be in scope
   */
  async close(): Promise<boolean> {
    return await invoke("plugin:sql|close", {
      db: this.path
    });
  }
}
