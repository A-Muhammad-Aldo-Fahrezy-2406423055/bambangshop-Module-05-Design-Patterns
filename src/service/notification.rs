use crate::model::product::Product;
use crate::model::subscriber::Subscriber;
use crate::model::notification::Notification;
use crate::repository::subscriber::SubscriberRepository;

pub struct NotificationService;

impl NotificationService {
    pub fn subscribe(product_type: &str, subscriber: Subscriber) -> Subscriber {
        SubscriberRepository::add(product_type, subscriber)
    }
}
