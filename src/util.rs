use std::os::unix::fs::{MetadataExt, chown};
use std::path::Path;
use std::{fs, io};

use color_eyre::Result;

pub fn create_dir_with_metadata(dir: &Path, meta: &fs::Metadata) -> Result<()> {
	if !dir.exists() {
		fs::create_dir(dir)?;
		set_metadata(dir, meta)?;
	}
	Ok(())
}

pub fn copy_file_with_owner(src: &Path, dst: &Path) -> Result<()> {
	fs::copy(src, dst)?;
	set_metadata(dst, &fs::metadata(src)?)?;
	Ok(())
}

pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
	if !dst.exists() {
		fs::create_dir_all(dst)?;
		set_metadata(dst, &fs::metadata(src)?)?;
	}

	for entry in fs::read_dir(src)? {
		let entry = entry?;
		let src_path = entry.path();
		let dst_path = dst.join(entry.file_name());

		if src_path.is_dir() {
			copy_dir_recursive(&src_path, &dst_path)?;
		} else {
			copy_file_with_owner(&src_path, &dst_path)?;
		}
	}
	Ok(())
}

fn set_metadata(path: &Path, metadata: &fs::Metadata) -> Result<()> {
	if !path.exists() {
		return Err(
			io::Error::new(
				io::ErrorKind::NotFound,
				format!("Path does not exist: {}", path.display()),
			)
			.into(),
		);
	}

	let perm = metadata.permissions();
	let uid = metadata.uid();
	let gid = metadata.gid();

	fs::set_permissions(path, perm)?;
	chown(path, Some(uid), Some(gid))?;

	Ok(())
}
