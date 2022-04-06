use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use log;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> impl Responder {
    log::info!("Saving new subscriber details in the database");
    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.as_ref())
    .await;

    match result {
        Ok(_) => {
            log::info!("New subscriber details have been saved");
            HttpResponse::Ok()
        }
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError()
        }
    }
}
