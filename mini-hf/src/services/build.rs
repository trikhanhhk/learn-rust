use std::{
    ffi::OsStr, path::{Path, PathBuf}, vec
};

use serde::Serialize;
use tokio::fs;

#[derive(Debug, Serialize)]
pub enum EntryKind {
    File,
    Dir,
}

#[derive(Debug, Serialize)]
pub struct Entry {
    pub kind: EntryKind,
    pub name: String,
    pub path: String,
    pub size: u64,
    pub children: Vec<Entry>,
}

fn get_name(path: &Path) -> String {
    path.file_name()
        .unwrap_or_else(|| OsStr::new(""))
        .to_string_lossy()
        .to_string()
}

pub async fn process_node(path: PathBuf, curr_depth: usize, max_depth: usize) -> Entry {
    // is file
    if let Ok(metadata) = fs::metadata(&path).await {
        if metadata.is_file() {
            return Entry {
                kind: EntryKind::File,
                name: get_name(&path),
                path: path.to_string_lossy().to_string(),
                size: metadata.len(),
                children: vec![]
            };
        }
    }

    // is dir
    let mut children_paths = vec![];
    if let Ok(mut read_dir) = fs::read_dir(&path).await {
        while let Ok(Some(entry)) = read_dir.next_entry().await {
            children_paths.push(entry.path());
        }
    }

    // run concurrent
    let mut tasks = vec![];

    for p in children_paths {
        tasks.push(process_node(
            p,
            curr_depth + 1,
            max_depth,
        ));
    }

    let sub_entities: Vec<Entry> = futures::future::join_all(tasks).await;

    let total_size = sub_entities.iter().map(|e| e.size).sum();

    let children = if curr_depth < max_depth {
        sub_entities
    } else {
        vec![]
    };

    Entry {
        kind: EntryKind::Dir,
        name: get_name(&path),
        path: path.to_string_lossy().to_string(),
        size: total_size,
        children,
    }

}