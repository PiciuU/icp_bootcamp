use candid::{CandidType, Deserialize};

#[derive(Clone, CandidType, Deserialize)]
pub struct Config {
    pub max_title_length: u8,
    pub max_content_length: u16,
    pub max_tags_count: u8,
    pub tags: Vec<String>
}

impl Config {
    pub fn new() -> Self {
        Self {
            max_title_length: 150u8,
            max_content_length: 2000u16,
            max_tags_count: 3u8,
            tags: vec!["default".to_string()]
        }
    }
}