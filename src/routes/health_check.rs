use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    println!("Anybody???");
    HttpResponse::Ok()
}
