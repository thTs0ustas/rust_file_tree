use clap::Parser;
use file_tree::{dir_walk, print_tree};
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, default_value = ".")]
    path: String,
}

fn main() {
    let args = Args::parse();
    let root = PathBuf::from(args.path);

    let tree = dir_walk(&root).unwrap();

    print_tree(root.to_str().unwrap(), &tree);
}
