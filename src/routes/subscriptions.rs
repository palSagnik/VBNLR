use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData
{
    email: String,
    name: String,
}

//using Form Extractor's syntax from actix_web
pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse
{
    HttpResponse::Ok().finish()
}
