use rand::Rng;
use redis::{Commands, RedisError};
use std::time::Duration;
use tokio::time;

async fn subscriber_task(
    client: redis::Client,
    channel: String,
    number: i32,
) -> Result<redis::Msg, RedisError> {
    println!("Starting subscriber n°{number}");
    let mut subscriber_con = client.get_connection()?;

    let mut pub_sub = subscriber_con.as_pubsub();
    pub_sub.subscribe(&channel)?;

    loop {
        let msg = match pub_sub.get_message() {
            Ok(msg) => msg,
            Err(err) => {
                println!("[Subscriber {}] - Error get_message(): {}", number, err);
                continue;
            }
        };
        // let msg = pub_sub.get_message().unwrap();
        let payload: String = msg.get_payload().unwrap();
        let channel: String = msg.get_channel().unwrap();

        println!(
            "[Subscriber {}] - Message received: '{}' from channel '{}'",
            number, payload, channel
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), RedisError> {
    let channel = String::from("channel-test");
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    println!("Redis client information: {:?}", client);

    let mut tasks = Vec::new();
    for number in 0..10 {
        let task = tokio::spawn(subscriber_task(client.clone(), channel.clone(), number));
        tasks.push(task);
    }

    let mut publisher_con = client.get_connection().unwrap();

    loop {
        println!("loop!");
        let message = format!("message {}", rand::thread_rng().gen_range(0..100));
        publisher_con.publish(&channel, String::from(message))?;
        time::sleep(Duration::from_secs(5)).await;
    }
}
