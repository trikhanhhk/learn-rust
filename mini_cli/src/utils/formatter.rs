use crate::services::build::Node;

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

pub fn format_tree(node: &Node) -> String {
    let mut result = String::new();
    let len = node.children.as_ref().map_or(0, |children| children.len());
    
    result.push_str(&format!(
        "{} {} {} ({})\n",
        node.is_file.then(|| "File").unwrap_or("Dir"),
        node.name,
        node.path.as_ref().map_or_else(|| "".to_string(), |p| format!("{} ", p)),
        format_size(node.size)
    ));

    if len > 0 {
        for (_i, child) in node.children.as_ref().unwrap().iter().enumerate() {
            result.push_str(&format_tree(child));
        }
    }
    result
}