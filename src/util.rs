use std::fs;
use std::path::Path;

use color_eyre::Result;

pub fn copy_file_with_owner(src: &Path, dst: &Path) -> Result<()> {
    fs::copy(src, dst)?;
    use std::os::unix::fs::MetadataExt;
    use std::os::unix::fs::chown;

    let metadata = fs::metadata(&src)?;
    let uid = metadata.uid();
    let gid = metadata.gid();

    chown(dst, Some(uid), Some(gid))?;
    Ok(())
}

pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
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
