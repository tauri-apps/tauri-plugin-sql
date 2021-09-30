import { invoke } from '@tauri-apps/api/tauri'

export interface QueryResult {
  rowsAffected: number
  lastInsertId: number
}

export default class Database {
  path: string
  constructor(path: string) {
    this.path = path
  }

  static load(path: string): Promise<Database> {
    return invoke('plugin:sql|load', {
      db: path
    }).then(() => new Database(path))
  }

  static get(path: string) {
    return new Database(path)
  }

  execute(query: string, bindValues?: any[]): Promise<QueryResult> {
    return invoke<[number, number]>('plugin:sql|execute', {
      db: this.path,
      query,
      values: bindValues ?? []
    }).then(([rowsAffected, lastInsertId]) => ({ rowsAffected, lastInsertId }))
  }

  select<T>(query: string, bindValues?: any[]): Promise<T> {
    return invoke('plugin:sql|select', {
      db: this.path,
      query,
      values: bindValues ?? []
    })
  }
}
