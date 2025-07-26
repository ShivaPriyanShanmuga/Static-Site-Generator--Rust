use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PageMetadata {
    pub title: Option<String>,
    pub date: Option<String>,  
}

pub struct Page {
    pub metadata: PageMetadata,
    pub content: String,
    pub html: String,
    pub source_path: std::path::PathBuf,
}