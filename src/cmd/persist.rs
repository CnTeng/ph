use std::fmt::Write as _;
use std::fs;
use std::os::unix::fs::{MetadataExt as _, PermissionsExt as _};
use std::path::Path;

use color_eyre::Result;
use owo_colors::OwoColorize as _;
use users::{get_group_by_gid, get_user_by_uid};

use crate::entry::persist_entry;

pub fn persist(root: &Path, entry: &Path) -> Result<()> {
    let dst = persist_entry(root, entry)?;

    println!(
        "Persist {} to {}",
        entry.display().bold(),
        dst.display().bold()
    );

    println!("Source:");

    print_metadata("target", &entry)?;
    if let Some(src_parent) = entry.parent() {
        print_metadata("parent", src_parent)?;
    }

    println!("Destination:");
    print_metadata("target", &dst)?;
    if let Some(dst_parent) = dst.parent() {
        print_metadata("parent", dst_parent)?;
    }

    Ok(())
}

fn print_metadata(info: &str, path: &Path) -> Result<()> {
    let meta = fs::metadata(path)?;
    let perm: fs::Permissions = meta.permissions();

    println!(
        "\t{}: {} {} {}",
        info,
        print_permissions(&perm),
        print_user_group(&meta),
        path.display().bold()
    );

    Ok(())
}

fn print_permissions(perm: &fs::Permissions) -> String {
    let mut s = String::new();
    for i in (0..3).rev() {
        let shift = i * 3;
        let bits = (perm.mode() >> shift) & 0b111;
        s.push(if bits & 0b100 != 0 { 'r' } else { '-' });
        s.push(if bits & 0b010 != 0 { 'w' } else { '-' });
        s.push(if bits & 0b001 != 0 { 'x' } else { '-' });
    }

    write!(&mut s, " {:04o}", perm.mode() & 0o777).unwrap();
    s
}

fn print_user_group(meta: &fs::Metadata) -> String {
    let uid = meta.uid();
    let gid = meta.gid();

    let user = get_user_by_uid(uid)
        .and_then(|u| u.name().to_str().map(String::from))
        .unwrap_or(uid.to_string());

    let group = get_group_by_gid(gid)
        .and_then(|g| g.name().to_str().map(String::from))
        .unwrap_or(gid.to_string());

    format!("{} {}", user, group)
}
