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

    pub fn notify(product_type: &str, status: &str, product: Product) {
        let subscribers = SubscriberRepository::list_all(product_type);
        for subscriber in subscribers {
            let payload = Notification {
                product_title: product.title.clone(),
                product_type: product.product_type.clone(),
                product_url: product.get_url(),
                subscriber_name: subscriber.name.clone(),
                status: String::from(status),
            };
            subscriber.update(payload);
        }
    }
}
