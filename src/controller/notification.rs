use rocket::{get, post, serde::json::Json};
use crate::service::notification::NotificationService;
use crate::model::subscriber::Subscriber;

#[post("/subscribe/<product_type>", format = "json", data = "<subscriber>")]
pub fn subscribe(product_type: &str, subscriber: Json<Subscriber>) -> Json<Subscriber> {
    let subscriber = NotificationService::subscribe(product_type, subscriber.into_inner());
    Json(subscriber)
}

#[post("/unsubscribe/<product_type>?<url>")]
pub fn unsubscribe(product_type: &str, url: &str) -> Json<Subscriber> {
    let subscriber = NotificationService::unsubscribe(product_type, url);
    Json(subscriber.unwrap())
}
