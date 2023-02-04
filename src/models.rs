use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct SearchResult {
    pub category: String,
    pub name: String,
    pub download_link: String,
    pub magnet_link: String
}

