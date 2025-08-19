use std::os::unix::fs::{MetadataExt, chown};
use std::path::Path;
use std::{fs, io};

pub fn create_dir_with_metadata(dir: &Path, meta: &fs::Metadata) -> io::Result<()> {
    if !dir.exists() {
        fs::create_dir(dir)?;
        set_metadata(dir, meta)?;
    }
    Ok(())
}

pub fn copy_file_with_metadata(src: &Path, dst: &Path) -> io::Result<()> {
    fs::copy(src, dst)?;
    set_metadata(dst, &fs::metadata(src)?)?;
    Ok(())
}

pub fn copy_dir_recursive_with_metadata(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
        set_metadata(dst, &fs::metadata(src)?)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive_with_metadata(&src_path, &dst_path)?;
        } else {
            copy_file_with_metadata(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

fn set_metadata(path: &Path, metadata: &fs::Metadata) -> io::Result<()> {
    let perm = metadata.permissions();
    let uid = metadata.uid();
    let gid = metadata.gid();

    fs::set_permissions(path, perm)?;
    chown(path, Some(uid), Some(gid))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_create_dir_with_metadata() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src");
        let dst = dir.path().join("dst");

        fs::create_dir(&src).unwrap();
        let meta = fs::metadata(&src).unwrap();
        create_dir_with_metadata(&dst, &meta).unwrap();
        let dst_meta = fs::metadata(&dst).unwrap();

        assert_eq!(dst_meta.permissions().mode(), meta.permissions().mode());
    }

    #[test]
    fn test_copy_file_with_metadata() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src.txt");
        let dst = dir.path().join("dst.txt");

        fs::write(&src, "Hello, World!").unwrap();
        let meta = fs::metadata(&src).unwrap();
        copy_file_with_metadata(&src, &dst).unwrap();
        let dst_meta = fs::metadata(&dst).unwrap();

        assert_eq!(dst_meta.permissions().mode(), meta.permissions().mode());
        assert_eq!(fs::read_to_string(&dst).unwrap(), "Hello, World!");
    }

    #[test]
    fn test_copy_dir_recursive_with_metadata() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src");
        let dst = dir.path().join("dst");

        fs::create_dir(&src).unwrap();
        fs::write(src.join("file1.txt"), "File 1").unwrap();
        fs::write(src.join("file2.txt"), "File 2").unwrap();
        copy_dir_recursive_with_metadata(&src, &dst).unwrap();
        let src_meta = fs::metadata(&src).unwrap();
        let dst_meta = fs::metadata(&dst).unwrap();

        assert_eq!(dst_meta.permissions().mode(), src_meta.permissions().mode());
        assert_eq!(fs::read_to_string(dst.join("file1.txt")).unwrap(), "File 1");
        assert_eq!(fs::read_to_string(dst.join("file2.txt")).unwrap(), "File 2");
    }
}
