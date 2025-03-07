use walkdir::WalkDir;

use std::process::Command;

#[derive(PartialEq, Eq, Debug)]
pub enum ExecOption {
	None,
	ForceFallbackWalkdir
}

pub fn folder_size(path: &str, opt: ExecOption) -> u64 {
	let (args, multiplier) = if cfg!(target_os = "linux") {
		(vec!["-s", "-b", path], 1)
	} else if cfg!(target_os = "macos") {
		(vec!["-s", "-k", path], 1024)
	} else {
		(vec!["-s", "-b", path], 1)
	};

	let res = match opt {
		ExecOption::ForceFallbackWalkdir => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "du not found")),
		_ => Command::new("du").args(&args).output()
	};
	
	//if opt == ExecOption::None && res.is_err() {
	//	println!("folder_size: {:?}", res);
	//	panic!();
	//}

	match res {
		Ok(output) if output.status.success() => {
			let stdout = String::from_utf8_lossy(&output.stdout);
			let size_str = stdout.split_whitespace().next().unwrap_or_default();
			return size_str
				.parse::<u64>()
				.map(|n| n * multiplier)
				.unwrap();
		}
		_ => {
			eprintln!("[folder_size] Warning: 'du' command not found or failed, falling back to walkdir.");
			// ls path
			//let output = Command::new("ls").arg(path).output().unwrap();
			//println!("ls: {:?}", output);
    		// 通用回退方案或 Windows 实现
    		let total = WalkDir::new(path)
    		    .into_iter()
    		    .filter_map(|e| e.ok())
    		    .filter_map(|e| e.metadata().ok())
    		    .filter(|md| md.is_file())
				//.filter(|md| { println!("[{}]", md.len()); true } )
    		    .fold(0, |acc, md| acc + md.len());

    		return total
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
	use std::path::Path;
	
	fn create_dir(dir_path: &str) {
        // 创建文件 (5字节)
        let file_path = Path::new(dir_path).join("test.txt");
        std::fs::write(&file_path, b"Hello").unwrap();

        // 创建子目录并添加文件 (200字节)
        let sub_dir = Path::new(dir_path).join("sub");
        std::fs::create_dir(&sub_dir).unwrap();
        std::fs::write(sub_dir.join("data.bin"), vec![0u8; 200]).unwrap();
	}

    #[test]
    fn cmd_folder_size() {
        // 创建临时目录
		let dir = tempdir().unwrap();
        let dir_path = dir.path().to_str().unwrap();
		create_dir(dir_path);

        // 计算文件夹大小
        let size = folder_size(&dir_path, ExecOption::None);

        // 验证总大小是否正确 (5 + 200 = 205)
        assert_eq!(size, 205);
    }
	#[test]
	fn cmd_folder_size_on_two_methods() {
        // 创建临时目录
		let dir = tempdir().unwrap();
        let dir_path = dir.path().to_str().unwrap();
		create_dir(dir_path);
		
        let size1 = folder_size(&dir_path, ExecOption::None);
        let size2 = folder_size(&dir_path, ExecOption::ForceFallbackWalkdir);
		
		panic!();

		assert_eq!(size1, size2);
	}
}