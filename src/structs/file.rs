pub mod file_type {
    use std::fs::Metadata;

    #[derive(Debug)]
    pub struct File {
        pub name: String,
        pub metadata: Metadata,
    }
}
