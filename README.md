# Tauri Plugin SQL

![Test](https://github.com/tauri-apps/tauri-plugin-sql/workflows/Test/badge.svg)

This plugin provides a "classical" Tauri Plugin Interface to SQL databases through [sqlx](https://github.com/launchbadge/sqlx).
It supports the `sqlite`, `mysql` and `postgres` drivers, enabled through a Cargo feature.

## Architecture

This repo shape might appear to be strange, but it is really just a hybrid Rust / Typescript project that recommends a specific type of consumption, namely using GIT as the secure distribution mechanism, and referencing specific unforgeable git hashes. Of course, it can also be consumed via Cargo and NPM.

### `/src`

Rust source code that contains the plugin definition and `sqlx` features.

### `/webview-src`

Typescript source for the /webview-dist folder that provides an API to interface with the rust code.

### `/webview-dist`

Tree-shakeable transpiled JS to be consumed in a Tauri application.

### `/bindings`

Forthcoming tauri bindings to other programming languages, like DENO.

## Installation

There are three general methods of installation that we can recommend.

1. Pull sources directly from Github using git tags / revision hashes (most secure, good for development, shown below)
2. Git submodule install this repo in your tauri project and then use `file` protocol to ingest the source
3. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)

For more details and usage see [the Todo app](examples/todos-app/). Please note, below in the dependencies you can also lock to a revision/tag in both the `Cargo.toml` and `package.json`

### RUST

`src-tauri/Cargo.toml`

```yaml
[dependencies.tauri-plugin-sql]
git = "https://github.com/tauri-apps/tauri-plugin-sql"
# tag = "v0.1.0" Not available yet
features = ["sqlite"] # or "postgres", or "mysql"
```

Use in `src-tauri/src/main.rs`:

```rust
use tauri_plugin_sql::TauriSql;

fn main() {
    tauri::Builder::default()
        .plugin(TauriSql::default())
        .build()
        .run();
}
```

### WEBVIEW

`Install from a tagged release`

**This is not available yet**

```sh
npm install github:tauri-apps/tauri-plugin-sql#v0.1.0
# or
yarn add github:tauri-apps/tauri-plugin-sql#v0.1.0
```

`Install from a commit`

```sh
npm install github:tauri-apps/tauri-plugin-sql#488558717b77d8a2bcb37acfd2eca9658aeadc8e
# or
yarn add github:tauri-apps/tauri-plugin-sql#488558717b77d8a2bcb37acfd2eca9658aeadc8e
```

`Install from a branch (dev)`

```sh
npm install github:tauri-apps/tauri-plugin-sql#dev
# or
yarn add github:tauri-apps/tauri-plugin-sql#dev
```

`package.json`

```json
  "dependencies": {
    "tauri-plugin-sql-api": "github:tauri-apps/tauri-plugin-sql",
```

Use within your JS/TS:

```ts
import Database from 'tauri-plugin-sql-api'

// sqlite. The path is relative to `tauri::api::path::BaseDirectory::App`.
const db = await Database.load('sqlite:test.db')
// mysql
const db = await Database.load('mysql://user:pass@host/database')
// postgres
const db = await Database.load('postgres://postgres:password@localhost/test')

await db.execute('INSERT INTO ...')
```

## License

MIT / Apache-2.0
