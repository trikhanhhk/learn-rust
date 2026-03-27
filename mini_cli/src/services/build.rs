use std::{fs, path::PathBuf, thread, vec};


#[derive(Debug)]
pub enum EntryKind {
    File,
    Dir,
}
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

pub fn build(root: PathBuf) -> Vec<Entry> {
    let mut current_level = vec![root];
    let mut results = vec![];

    while !current_level.is_empty() {
        let mut next_level = vec![];

        for chunk in current_level.chunks(10) { // max 10 threads at a time
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