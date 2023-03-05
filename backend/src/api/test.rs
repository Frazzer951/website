use actix_web::{get, web::Json};

#[get("/test")]
pub async fn test() -> Json<String> {
    Json("test".to_string())
}
