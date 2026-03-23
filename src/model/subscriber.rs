use rocket::serde::{Deserialize, Serialize};
use rocket::log;
use rocket::serde::json::to_string;
use rocket::tokio;
use bambangshop::REQWEST_CLIENT;
use crate::model::notification::Notification;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber {
    pub url: String,
    pub name: String,
}

impl Subscriber {
    pub fn update(&self, payload: Notification) {
        let url = self.url.clone();
        tokio::spawn(async move {
            match REQWEST_CLIENT.post(&url).json(&payload).send().await {
                Ok(_) => println!("Successfully sent notification to {}", url),
                Err(e) => println!("Failed to send notification to {}: {}", url, e)
            }
        });
    }
}