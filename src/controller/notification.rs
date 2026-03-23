use rocket::serde::json::Json;
use bambangshop::{Result, compose_error_response};
use crate::model::subscriber::Subscriber;
use crate::service::notification::NotificationService;

#[post("/subscribe/<product_type>", data = "<subscriber>")]
pub fn subscribe(product_type: &str, subscriber: Json<Subscriber>) -> Result<Subscriber> {
    let result = NotificationService::subscribe(product_type, subscriber.into_inner());
    Ok(Json(result))
}

#[post("/unsubscribe/<product_type>?<url>")]
pub fn unsubscribe(product_type: &str, url: &str) -> Result<Subscriber> {
    let result = NotificationService::unsubscribe(product_type, url);
    match result {
        Some(subscriber) => Ok(Json(subscriber)),
        None => Err(compose_error_response(
            rocket::http::Status::NotFound,
            String::from("Subscriber not found."),
        )),
    }
}
