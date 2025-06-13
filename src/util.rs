use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::entry::PersistEntrySet;

pub fn copy_file_with_owner(src: &Path, dst: &Path) -> io::Result<()> {
    fs::copy(src, dst)?;
    use std::os::unix::fs::MetadataExt;
    use std::os::unix::fs::chown;

    let metadata = fs::metadata(&src)?;
    let uid = metadata.uid();
    let gid = metadata.gid();

    chown(dst, Some(uid), Some(gid))?;
    Ok(())
}

pub fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

pub fn collect_paths(
    dir: &Path,
    path_set: &PersistEntrySet,
    delete_paths: &mut Vec<PathBuf>,
) -> Result<(), io::Error> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        match path_set.path.get(&path) {
            Some(true) => continue, // This path is kept, skip it
            Some(false) => {
                if path.is_dir() {
                    collect_paths(&path, path_set, delete_paths)?;
                }
            }
            None => delete_paths.push(path),
        }
    }
    Ok(())
}
