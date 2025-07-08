use actix_web::{HttpResponse, Responder, post};
use actix_multipart::Multipart;
use crate::services::upload::save_file;

#[post("/upload")]
pub async fn upload_handler(payload: Multipart) -> impl Responder {
    let upload_status = save_file(payload).await;
    
    match upload_status {
        Some(upload_status) => HttpResponse::Ok().json(upload_status),
        _ => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}