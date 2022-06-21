// use std::env;
use lettre::*;
// extern crate dotenv;

// dotenv::dotenv().expect("Failed to read .env file");

// static user_address: &str = env::var("USER_ADDRESS").expect("USER_ADDRESS not found");
// static email: &str = env::var("PASSWORD").expect("PASSWORD not found");


pub fn message_maker(body: &str, receiver: &str) -> Message {
    return Message::builder()
            .from("babaali196@gmail.com".parse().unwrap())
            .reply_to("babaali196@gmail.com".parse().unwrap())
            .to(receiver.parse().unwrap())
            .subject("PTGEON mail service")
            .body(String::from(body))
            .unwrap()
}
