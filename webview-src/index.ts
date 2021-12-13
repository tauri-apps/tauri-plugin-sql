import { invoke } from '@tauri-apps/api/tauri'

export interface QueryResult {
  /** the number of rows affected by the query. */
  rowsAffected: number
  /**
   * The last inserted `id`.
   *
   * This value is always `0` when using the Postgres driver. If the
   * last inserted id is required on Postgres, the `select` function
   * must be used, with a `RETURNING` clause
   * (`INSERT INTO todos (title) VALUES ($1) RETURNING id`).
   */
  lastInsertId: number
}

/**
 * **Database**
 *
 * the database class serves as the primary interface for the frontend
 * to communicate to the backend's `tauri-plugin-sql` API.
 */
export default class Database {
  path: string
  constructor(path: string) {
    this.path = path
  }

  /**
   * **load**
   *
   * A static initializer which connects to the underlying database
   * and returns a `Database` instance once a connecion to the database
   * is established.
   * ```ts
   * const db = await Database.load("sqlite:test.db");
   * ```
   */
  static async load(path: string): Promise<Database> {
    return await invoke<string>('plugin:sql|load', {
      db: path
    }).then((p) => new Database(p))
  }

  /**
   * **get**
   *
   * A static initializer which synchronously returns an instance of
   * the Database class while deferring the actual database connection
   * until the first invokation or selection on the database.
   *
   * ```ts
   * const db = Database.get("sqlite:test.db");
   * ```
   */
  static get(path: string): Database {
    return new Database(path)
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
  async execute(query: string, bindValues?: any[]): Promise<QueryResult> {
    return await invoke<[number, number]>('plugin:sql|execute', {
      db: this.path,
      query,
      values: bindValues ?? []
    }).then(([rowsAffected, lastInsertId]) => ({ rowsAffected, lastInsertId }))
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
  async select<T>(query: string, bindValues?: any[]): Promise<T> {
    return await invoke('plugin:sql|select', {
      db: this.path,
      query,
      values: bindValues ?? []
    })
  }
}
