use std::time::Duration;

use log::info;

use rdkafka::config::ClientConfig;
// use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};

pub async fn produce(brokers: &str, topic_name: &str, msg: &Vec<u8>, id: &str) {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

        // if id type is i64
    // let id_string = id.to_string();
    // let id_str = id_string.as_str();

    let futures = (0..1)
        .map(|i|  async move {
            // The send operation on the topic returns a future, which will be
            // completed once the result or failure from Kafka is received.
            let delivery_status = producer
                .send(
                    FutureRecord::to(topic_name)
                        .payload(msg)
                        .key(id)
                        // .headers(OwnedHeaders::new().insert(Header {
                        //     key: "header_key",
                        //     value: Some("header_value"),
                        // }))
                        ,
                    Duration::from_secs(0),
                )
                .await;

            // This will be executed when the result is received.
            info!("Delivery status for message {} received", i+1);
            delivery_status
        })
        .collect::<Vec<_>>();

    // This loop will wait until all delivery statuses have been received.
    for future in futures {
        info!("Future completed. Result: {:?}", future.await);
    }
}
