use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn add(path: PathBuf, root: PathBuf) -> Result<(), io::Error> {
    if path.exists() {
        eprintln!("Error: {} does not exist.", path.display());
    }
    let source_abs = fs::canonicalize(&path)?;

    let dest_dir = root.join(source_abs.strip_prefix("/").unwrap_or(&source_abs));

    let parent_dir = dest_dir.parent().unwrap();

    fs::create_dir_all(&parent_dir)?;

    if source_abs.is_dir() {
        copy_dir_recursive(&source_abs, &dest_dir)?;
    } else {
        fs::copy(&source_abs, &dest_dir)?;
    };

    println!("Persist {} to {}", source_abs.display(), dest_dir.display());
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
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
