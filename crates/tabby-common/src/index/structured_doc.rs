pub mod fields {
    pub const KIND: &str = "kind";

    pub mod web {
        pub const TITLE: &str = "title";
        pub const LINK: &str = "link";
        pub const CHUNK_TEXT: &str = "chunk_text";
    }

    pub mod issue {
        pub const TITLE: &str = "title";
        pub const LINK: &str = "link";
        pub const BODY: &str = "body";
        pub const CLOSED: &str = "closed";
    }
}
