pub mod directory_type {
    use crate::FileTree;

    #[derive(Debug)]
    pub struct Directory {
        pub name: String,
        pub entries: Vec<FileTree>,
    }
}
