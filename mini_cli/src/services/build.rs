use std::{
    ffi::OsStr, fs, io, os::unix::process, path::{Path, PathBuf}, sync::{Arc, mpsc}, thread, vec
};

use rayon::prelude::*;

#[derive(Debug)]
pub enum EntryKind {
    File,
    Dir,
}

#[derive(Debug)]
pub struct Entry {
    pub kind: EntryKind,
    pub name: String,
    pub path: String,
    pub size: u64,
}

fn process_folder(path: PathBuf) -> (Vec<PathBuf>, Vec<Entry>) {
    let mut sub_dirs = vec![];
    let mut entries = vec![];

    entries.push(Entry {
        kind: EntryKind::Dir,
        name: path.file_name().unwrap().to_string_lossy().to_string(),
        path: path.to_string_lossy().to_string(),
        size: 0,
    });

    if let Ok(read_dir) = fs::read_dir(&path) {
        for entry in read_dir.flatten() {
            let p = entry.path();
            
            if p.is_dir() {
                sub_dirs.push(p);
            } else {
                let size = fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
                entries.push(Entry {
                    kind: EntryKind::File,
                    name: p.file_name().unwrap().to_string_lossy().to_string(),
                    path: p.to_string_lossy().to_string(),
                    size,
                });
            }
        }
    }

    (sub_dirs, entries)
}

pub fn build_threads(root: PathBuf, max_threads: usize) -> Vec<Entry> {
    let mut current_level = vec![root];
    let mut results = vec![];

    while !current_level.is_empty() {
        let mut next_level = vec![];

        for chunk in current_level.chunks(max_threads) { // max_threads at a time
            let mut handles = vec![];
            for path in chunk {
                let path = path.clone();

                let handle = thread::spawn(move || {
                    process_folder(path)
                });
                handles.push(handle);
            }

            for handle in handles {
                if let Ok((dirs, entries)) = handle.join() {
                    results.extend(entries);
                    next_level.extend(dirs);
                }
            }
        }
        current_level = next_level;
    }
    results
}

// build using rayon
pub fn build_rayon(root: PathBuf) -> Vec<Entry> {
    let mut results = vec![];
    let mut current_level = vec![root];

    while !current_level.is_empty() {
        let mut next_level = vec![];

        // using rayon to map
        let entries: Vec<_> = current_level
            .par_iter()
            .map(|path| process_folder(path.clone()))
            .collect();

        for (dirs, entries) in entries {
            results.extend(entries);
            next_level.extend(dirs);
        }
        current_level = next_level;
    }
    results
}

pub struct Node {
    pub name: String,
    pub size: u64,
    pub children: Option<Vec<Node>>,
    pub is_file: bool,
    pub path: Option<String>,
}

pub fn build_single(path: &Path) -> io::Result<Node> {
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

            match build_single(&child_path) {
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
        name: path.file_name().unwrap_or_else(|| std::ffi::OsStr::new("")).to_string_lossy().to_string(),
        size,
        children: if children.is_empty() { None } else { Some(children) },
        is_file: false,
        path: Some(path.to_string_lossy().to_string()),
    })
}