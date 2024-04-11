pub mod structs;
use colored::Colorize;

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

pub fn print_tree(root: &str, dir: &Directory) {
    const OTHER_CHILD: &str = "│   "; // prefix: pipe
    const OTHER_ENTRY: &str = "├── "; // connector: tee
    const FINAL_CHILD: &str = "    "; // prefix: no more siblings
    const FINAL_ENTRY: &str = "└── "; // connector: elbow

    println!("{}", root);
    let (d, f) = visit(dir, "");
    println!("\n{} directories, {} files", d, f);

    fn visit(node: &Directory, prefix: &str) -> (usize, usize) {
        let mut dirs = 1;
        let mut files = 0;
        let mut entries = node.entries.len();

        for entry in &node.entries {
            entries -= 1;
            let connector = if entries == 0 {
                FINAL_ENTRY
            } else {
                OTHER_ENTRY
            };
            match entry {
                FileTree::LinkNode(symlink) => {
                    println!(
                        "{}{}{} -> {:?}",
                        prefix,
                        connector,
                        symlink.name.bright_blue(),
                        symlink.target
                    );
                    files += 1;
                }
                FileTree::FileNode(file) => {
                    println!("{}{}{}", prefix, connector, file.name.purple());
                    files += 1;
                }
                FileTree::DirNode(directory) => {
                    println!("{}{}{}", prefix, connector, directory.name.yellow());
                    let new_connector = format!(
                        "{}{}",
                        prefix,
                        if entries == 0 {
                            FINAL_CHILD
                        } else {
                            OTHER_CHILD
                        },
                    );
                    let (sub_dirs, sub_files) = visit(directory, &new_connector);
                    dirs += sub_dirs;
                    files += sub_files;
                }
            }
        }
        (dirs, files)
    }
}
