use walkdir::WalkDir;

use std::process::Command;

pub fn folder_size(path: &str) -> Result<u64, String> {
	let (args, multiplier) = if cfg!(target_os = "linux") {
		(vec!["-s", "-b", path], 1)
	} else if cfg!(target_os = "macos") {
		(vec!["-s", "-k", path], 1024)
	} else {
		(vec!["-s", "-b", path], 1)
	};

	match Command::new("du").args(&args).output() {
		Ok(output) if output.status.success() => {
			let stdout = String::from_utf8_lossy(&output.stdout);
			let size_str = stdout.split_whitespace().next().unwrap_or_default();
			return size_str
				.parse::<u64>()
				.map(|n| n * multiplier)
				.map_err(|_| "Failed to parse du output".into());
		}
		_ => {
			eprintln!("[folder_size] Warning: 'du' command not found or failed, falling back to walkdir.");
    		// 通用回退方案或 Windows 实现
    		let total = walkdir::WalkDir::new(path)
    		    .into_iter()
    		    .filter_map(|e| e.ok())
    		    .filter_map(|e| e.metadata().ok())
    		    .filter(|md| md.is_file())
    		    .fold(0, |acc, md| acc + md.len());

    		return Ok(total)
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn cmd_folder_size() {
        // 创建临时目录
        let dir = tempdir().unwrap();
        let dir_path = dir.path().to_str().unwrap();

        // 创建文件 (5字节)
        let file_path = dir.path().join("test.txt");
        std::fs::write(&file_path, b"Hello").unwrap();

        // 创建子目录并添加文件 (200字节)
        let sub_dir = dir.path().join("sub");
        std::fs::create_dir(&sub_dir).unwrap();
        std::fs::write(sub_dir.join("data.bin"), vec![0u8; 200]).unwrap();

        // 计算文件夹大小
        let size = folder_size(dir_path);

        // 验证总大小是否正确 (5 + 200 = 205)
        assert_eq!(size, 205);
    }
}