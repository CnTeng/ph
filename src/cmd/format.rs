use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};

use owo_colors::OwoColorize as _;
use uzers::{get_group_by_gid, get_user_by_uid};

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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    use super::*;

    #[test]
    fn test_format_permissions() {
        let perm = fs::Permissions::from_mode(0o754);
        let s = format_permissions(&perm);
        assert_eq!(s, "rwxr-xr-- 0754");
    }

    #[test]
    fn test_format_user_group_fallback() {
        let s = format_user_group(99999, 99999);
        assert_eq!(s, "99999 99999");
    }

    #[test]
    fn test_format_entry_set() {
        let entry_set: PersistEntrySet = vec![
            "/path/to/entry1".into(),
            "/path/to/entry2".into(),
            "/path/to/entry3".into(),
        ]
        .into_iter()
        .collect();

        let formatted = format_entry_set(&entry_set);
        assert_eq!(
            formatted,
            "\t\u{1b}[31m/path/to/entry1\u{1b}[39m\n\t\u{1b}[31m/path/to/entry2\u{1b}[39m\n\t\u{1b}[31m/path/to/entry3\u{1b}[39m"
        );
    }
}
