use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::Serialize;

#[derive(Serialize)]
pub struct FileMetadata {
    pub file_id: Uuid,
    pub original_name: String,
    pub chunk_hashes: Vec<String>,
    pub chunk_count: usize,
    pub upload_time: DateTime<Utc>
}