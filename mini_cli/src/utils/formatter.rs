use crate::services::build::{Entry, EntryKind};

pub fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if size >= TB {
        format!("{:.2} TB", size as f64 / TB as f64)
    } else if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{:.2} B", size as f64)
    }
}

pub fn get_kind_str(kind: &EntryKind) -> &'static str {
    match kind {
        EntryKind::File => "File",
        EntryKind::Dir => "Dir",
    }
}

pub fn format_tree(entry: &Entry) -> String {
    let mut result = String::new();
    
    result.push_str(&format!(
        "{} {} {} ({})\n",
        get_kind_str(&entry.kind),
        entry.name,
        entry.path,
        format_size(entry.size)
    ));

    let children = &entry.children;
    if !children.is_empty() {
        for child in children {
            result.push_str(&format_tree(&child));
        }
    }
    result
}