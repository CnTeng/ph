use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};

use owo_colors::OwoColorize as _;
use users::{get_group_by_gid, get_user_by_uid};

use crate::entry::PersistEntrySet;

pub fn format_entry_set(entry_set: &PersistEntrySet) -> String {
    entry_set
        .iter()
        .fold(String::new(), |mut s, e| {
            s.push('\t');
            s.push_str(&e.display().red().to_string());
            s.push('\n');
            s
        })
        .trim_end()
        .to_string()
}

pub fn format_metadata(meta: &fs::Metadata) -> String {
    format!(
        "{} {}",
        format_permissions(&meta.permissions()),
        format_user_group(meta.uid(), meta.gid()),
    )
}

fn format_permissions(perm: &fs::Permissions) -> String {
    let mut s = String::new();
    for i in (0..3).rev() {
        let shift = i * 3;
        let bits = (perm.mode() >> shift) & 0b111;
        s.push(if bits & 0b100 != 0 { 'r' } else { '-' });
        s.push(if bits & 0b010 != 0 { 'w' } else { '-' });
        s.push(if bits & 0b001 != 0 { 'x' } else { '-' });
    }

    format!("{} {:04o}", s, perm.mode() & 0o777)
}

fn format_user_group(uid: u32, gid: u32) -> String {
    let user = get_user_by_uid(uid)
        .and_then(|u| u.name().to_str().map(String::from))
        .unwrap_or(uid.to_string());

    let group = get_group_by_gid(gid)
        .and_then(|g| g.name().to_str().map(String::from))
        .unwrap_or(gid.to_string());

    format!("{user} {group}")
}
