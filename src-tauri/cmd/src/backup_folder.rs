//no warning
#![allow(warnings)]

use fs_extra;

pub fn backup_folder(src: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    //let mut options = fs_extra::dir::CopyOptions::new();
    //options.overwrite = true; // 如果目标路径已存在文件，则覆盖
    //fs_extra::dir::copy(src, dest, &options)?;
    //Ok(())
    Err("Not implemented yet".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
	use std::path::Path;
	
	#[test]
	fn cmd_backup_folder() {
	}
}