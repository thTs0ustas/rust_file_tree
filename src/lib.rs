pub mod structs;

use std::{
    cmp::Ordering,
    ffi::OsStr,
    fs::{metadata, read_dir, read_link, DirEntry},
    io::Result,
    path::PathBuf,
};

use structs::directory::directory_type::Directory;
use structs::file::file_type::File;
use structs::symlink::symlink_type::Symlink;

#[derive(Debug)]
pub enum FileTree {
    DirNode(Directory),
    FileNode(File),
    LinkNode(Symlink),
}

pub fn is_not_hidden(name: &str) -> bool {
    !name.starts_with('.')
}

pub fn sort_by_name(a: &DirEntry, b: &DirEntry) -> Ordering {
    let a_name: String = a.path().file_name().unwrap().to_str().unwrap().into();
    let b_name: String = b.path().file_name().unwrap().to_str().unwrap().into();
    a_name.cmp(&b_name)
}

pub fn dir_walk(root: &PathBuf) -> Result<Directory> {
    let mut entries: Vec<DirEntry> = read_dir(root)?.filter_map(|result| result.ok()).collect();

    entries.sort_by(sort_by_name);

    let mut directory: Vec<FileTree> = Vec::with_capacity(entries.len());

    for e in entries {
        let path = e.path();
        let name: String = path.file_name().unwrap().to_str().unwrap().into();

        if !is_not_hidden(&name) {
            continue;
        };

        let metadata = metadata(&path).unwrap();

        let node = match path {
            path if path.is_dir() => FileTree::DirNode(dir_walk(&root.join(name))?),
            path if path.is_symlink() => FileTree::LinkNode(Symlink {
                name,
                target: read_link(path).unwrap(),
                metadata,
            }),
            path if path.is_file() => FileTree::FileNode(File { name, metadata }),
            _ => unreachable!(),
        };
        directory.push(node);
    }
    let name = root
        .file_name()
        .unwrap_or(OsStr::new("."))
        .to_str()
        .unwrap()
        .into();

    Ok(Directory {
        name,
        entries: directory,
    })
}
