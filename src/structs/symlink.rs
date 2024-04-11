pub mod symlink_type {
    use std::fs::Metadata;
    use std::path::PathBuf;

    #[derive(Debug)]
    pub struct Symlink {
        pub name: String,
        pub metadata: Metadata,
        pub target: PathBuf,
    }
}
