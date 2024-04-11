use std::path::PathBuf;

use file_tree::dir_walk;

fn main() {
    let root = PathBuf::from("src");

    let tree = dir_walk(&root).unwrap();

    println!("{:#?}", tree);
}
