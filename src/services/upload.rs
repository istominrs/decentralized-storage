use actix_multipart::Multipart;
use futures_util::StreamExt;
use uuid::Uuid;
use chrono::Utc;
use crate::models::file_metadata::FileMetadata;

pub async fn save_file(mut payload: Multipart) -> Option<FileMetadata> {
   let mut file_bytes = Vec::new();
   let mut original_name = String::new();
   
   while let Some(item) = payload.next().await {
      match item {
         Ok(mut field) => {
            if let Some(disposition) = field.content_disposition() {
               if let Some(filename) = disposition.get_filename() {
                  original_name = filename.to_string();
               }
            }
            
            while let Some(chunk) = field.next().await {
               match chunk {
                  Ok(data) => file_bytes.extend_from_slice(&data),
                  Err(_) => return None,
               }
            }
         }
         Err(_) => return None,
      }
   }
   
   let chunk_size = 1024;
   let mut chunk_hashes = Vec::new();
   
   // TODO: add aes
   for chunk in file_bytes.chunks(chunk_size) {
      let hash = blake3::hash(chunk);
      chunk_hashes.push(format!("{}", hash.to_hex()));
   }
   
   let chunk_count = chunk_hashes.len();
   
   let file_metadata = FileMetadata {
      file_id: Uuid::new_v4(),
      original_name,
      chunk_hashes,
      chunk_count,
      upload_time: Utc::now(),
   };
   
   // TODO: add save to db

   Some(file_metadata)
}
