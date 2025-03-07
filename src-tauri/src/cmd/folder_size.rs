use walkdir::WalkDir;

pub fn folder_size(folder_path: &str) -> u64 {
    let mut size = 0;
    for entry in WalkDir::new(folder_path) {
        if let Ok(e) = entry {
            if e.file_type().is_file() {
                if let Ok(metadata) = e.metadata() {
                    size += metadata.len();
                }
            }
        }
    }
    size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmd_folder_size() {
		//create a text file
		//mocking the file creation
		
		let result = folder_size("src-tauri");
		assert_eq!(result, 0);
    }
}