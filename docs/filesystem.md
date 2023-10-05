# Interacting with the filesystem using Tauri

```rust
#[tauri::command]
fn file_exists(file: &PathBuf) -> bool {
    file.is_file()
}
```