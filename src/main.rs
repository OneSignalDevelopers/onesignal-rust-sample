extern crate dotenv;

use dotenv::dotenv;
use onesignal::{
    apis::configuration::Configuration,
    models::{Notification, StringMap},
    *,
};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    send_notification().await;
    println!("Done");
}

// API Client
fn create_configuration() -> Box<Configuration> {
    let mut configuration = apis::configuration::Configuration::new();
    configuration.app_key_token =
        Some(env::var("REST_API_KEY").expect("REST_API_KEY must be set!"));
    configuration.user_key_token =
        Some(env::var("USER_AUTH_KEY").expect("USER_AUTH_KEY must be set!"));

    Box::new(configuration)
}

fn create_notification() -> Box<Notification> {
    let mut notification = Notification::new(env::var("APP_ID").expect("APP_ID must be set!"));

    let mut string_map = StringMap::new();
    string_map.en = Some(String::from("Rust test notification"));
    notification.contents = Some(Box::new(string_map));
    notification.is_chrome_web = Some(true);
    notification.is_any_web = Some(true);
    notification.included_segments = Some(vec![String::from("Subscribed Users")]);

    Box::new(notification)
}

async fn send_notification() {
    // Prepare configuration and the notification objects
    let configuration = create_configuration();
    let notification = create_notification();

    // Send notification to the server
    let create_notification_response =
        apis::default_api::create_notification(&configuration, *notification).await;

    // Check the result
    if let Ok(ref created_notification) = create_notification_response {
        println!("Created notification id: {}", created_notification.id);
    }

    if let Err(ref created_notification_error) = create_notification_response {
        println!(
            "Created notification error: {}",
            created_notification_error.to_string()
        );
    }
}
