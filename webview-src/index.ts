import { invoke } from '@tauri-apps/api/tauri'

export default class Database {
  path: string
  constructor(path: string) {
    this.path = path
  }

  static load(path: string) {
    invoke('plugin:sql|sqlx_load', {
      db: path
    }).then(() => new Database(path))
  }

  static get(path: string) {
    return new Database(path)
  }

  execute(query: string): Promise<number> {
    return invoke<number>('plugin:sql|sqlx_execute', {
      db: this.path,
      query
    })
  }

  select<T>(query: string): Promise<T> {
    return invoke('plugin:sql|sqlx_select', {
      db: this.path,
      query
    })
  }
}
