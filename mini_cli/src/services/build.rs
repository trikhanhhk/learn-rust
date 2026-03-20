use std::{ffi::OsStr, fs, io, path::Path};

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub size: u64,
    pub children: Option<Vec<Node>>,
    pub is_file: bool,
    pub path: Option<String>,
}

pub fn build_tree(path: &Path) -> io::Result<Node> {
    let metadata = fs::symlink_metadata(path)?;

    if metadata.is_file() {
        return Ok(Node {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            size: metadata.len(),
            children: None,
            is_file: true,
            path: Some(path.to_string_lossy().to_string()),
        });
    } 

    let mut size = 0;
    let mut children = vec![];

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => {
            // skip if can't read
            return Ok(Node {
                name: path.file_name().unwrap_or_else(|| OsStr::new("")).to_string_lossy().to_string(),
                size: 0,
                children: None,
                is_file: false,
                path: Some(path.to_string_lossy().to_string()),
            });
        }
    };
    
    for entry in entries {
        if let Ok(entry) = entry {
            let child_path = entry.path();

            match build_tree(&child_path) {
                Ok(child_node) => {
                    size += child_node.size;
                    children.push(child_node);
                }
                Err(_) => {
                    // skip
                    continue;
                }
            }
        }
    }

    Ok(Node {
        name: path.file_name().unwrap_or_else(|| OsStr::new("")).to_string_lossy().to_string(),
        size,
        children: if children.is_empty() { None } else { Some(children) },
        is_file: false,
        path: Some(path.to_string_lossy().to_string()),
    })
}