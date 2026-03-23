use crate::model::product::Product;
use crate::model::subscriber::Subscriber;
use crate::model::notification::Notification;
use crate::repository::subscriber::SubscriberRepository;

pub struct NotificationService;

impl NotificationService {
    pub fn subscribe(product_type: &str, subscriber: Subscriber) -> Subscriber {
        SubscriberRepository::add(product_type, subscriber)
    }

    pub fn unsubscribe(product_type: &str, url: &str) -> Option<Subscriber> {
        SubscriberRepository::delete(product_type, url)
    }
}
