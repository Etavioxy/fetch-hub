use cmd;
mod serialize;

#[tauri::command]
fn human_read(serialized: &str, indent: usize, level: usize, linewidth: usize) -> String {
	return serialize::human_read(serialized, indent, level, linewidth);
}

#[tauri::command]
fn folder_size(folder_path: &str) -> u64 {
	cmd::folder_size::folder_size(folder_path, None)
}

#[tauri::command]
fn backup_folder(src: &str, dest: &str) -> String {
    let res = cmd::backup_folder::backup_folder(src, dest);
    match res {
        Ok(_) => "success".to_string(),
        Err(e) => format!("error: {}", e),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            human_read,
            folder_size,
            backup_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
