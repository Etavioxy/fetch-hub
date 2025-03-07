use cmd;
mod serialize;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn human_read(serialized: &str, indent: usize, level: usize, linewidth: usize) -> String {
	return serialize::human_read(serialized, indent, level, linewidth);
}

#[tauri::command]
fn folder_size(folder_path: &str) -> u64 {
	cmd::folder_size::folder_size(folder_path, cmd::folder_size::ExecOption::None)
}

#[tauri::command]
fn backup_folder(src: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true; // 如果目标路径已存在文件，则覆盖
    fs_extra::dir::copy(src, dest, &options)?;
    Ok(())
}

#[tauri::command]
fn backup_folder_1(src: &str, dest: &str) -> String {
    let res = backup_folder(src, dest);
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
            greet,
            human_read,
            folder_size,
            backup_folder_1
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
