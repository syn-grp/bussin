# Interacting with the filesystem using Tauri

### Check whether file exists using backend
```rust
#[tauri::command]
fn file_exists(file: &str) -> bool {
    PathBuf::from_str(file).is_ok_and(|f| f.is_file())
}
```

# todo

### List VST plugins
* Get list of plugins
* Accept base directory from frontend
* Store in state
* Unit test

### Partial VST Host implementation for correct plugin parsing
* Can fall back to fs implementation

### Simple .als file parse

### Fill .als file parse

### Simple .asd file parse
* See [AbletonParsing](https://github.com/DBraun/AbletonParsing/tree/main/src)

### Check audio and plugins exist for project

### Concurrency
* Do more things
* ???
* Profit

### Open project in Ableton from frontend

### Use some storage backend
* SQLite

### Automatically listen for changes to .als projects
* Accept a list of base directories to listen to
* Try using Tauri Plugin [tauri-plugin-fs-watch](https://github.com/tauri-apps/tauri-plugin-fs-watch)

### Track changes to a project over time à la git

