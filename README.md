# Tauri Plugin SQL
![Test](https://github.com/tauri-apps/tauri-plugin-sql/workflows/Test/badge.svg)

This plugin provides a "classical" Tauri Plugin Interface to SQL databases through [sqlx](https://github.com/launchbadge/sqlx).

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
1. Pull sources directly from Github using git tags / revision hashes (most secure, good for developement, shown below)
2. Git submodule install this repo in your tauri project and then use `file` protocol to ingest the source
3. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)

For more details and usage see [the Todo app](examples/todo/src/App.svelte). Please note, below in the dependencies you can also lock to a revision/tag in both the `Cargo.toml` and `package.json`

### RUST
`src-tauri/Cargo.toml`
```yaml
[dependencies.tauri-plugin-sql]
git = "https://github.com/tauri-apps/tauri-plugin-sql"
tag = "v0.1.0"
#branch = "main"
```

Use in `src-tauri/src/main.rs`:
```rust
use tauri_plugin_sql::TauriSql;

fn main() {
    tauri::AppBuilder::new()
        .plugin(TauriSql {})
        .build()
        .run();
}
```

### WEBVIEW
`Install from a tagged release`
```
npm install github:tauri-apps/tauri-plugin-sql#v0.1.0
# or
yarn add github:tauri-apps/tauri-plugin-sql#v0.1.0
```

`Install from a commit`
```
npm install github:tauri-apps/tauri-plugin-sql#6749525a47a95439c9703d3a49b94ac65660998f
# or
yarn add github:tauri-apps/tauri-plugin-sql#6749525a47a95439c9703d3a49b94ac65660998f
```

`package.json`
```json
  "dependencies": {
    "tauri-plugin-sql": "tauri-apps/tauri-plugin-sql#v0.1.0",
```

Use within your JS/TS:
```
import Database from 'tauri-plugin-sql'
const db = Database.get('sqlite:test.db')
await db.execute('INSERT INTO ...')
```

# License
MIT / Apache-2.0
