use std::time::Duration;
use std::u8;

use clap::{App, Arg};
use log::info;

use rdkafka::config::ClientConfig;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;

use crate::example_utils::setup_logger;

mod example_utils;

pub async fn produce(brokers: &str, topic_name: &str, msg: &Vec<u8>) {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");
        let id = 4.to_string();
        let id_string = id.as_str();

    let futures = (0..1)
        .map(|i|  async move {
            // The send operation on the topic returns a future, which will be
            // completed once the result or failure from Kafka is received.
            let delivery_status = producer
                .send(
                    FutureRecord::to(topic_name)
                        .payload(msg)
                        .key(id_string)
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

#[tokio::main]
async fn main() {
    // let matches = App::new("producer example")
    //     .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
    //     .about("Simple command line producer")
    //     .arg(
    //         Arg::with_name("brokers")
    //             .short("b")
    //             .long("brokers")
    //             .help("Broker list in kafka format")
    //             .takes_value(true)
    //             .default_value("localhost:9092"),
    //     )
    //     .arg(
    //         Arg::with_name("log-conf")
    //             .long("log-conf")
    //             .help("Configure the logging format (example: 'rdkafka=trace')")
    //             .takes_value(true),
    //     )
    //     .arg(
    //         Arg::with_name("topic")
    //             .short("t")
    //             .long("topic")
    //             .help("Destination topic")
    //             .takes_value(true)
    //             .required(true),
    //     )
    //     .get_matches();

    // setup_logger(true, matches.value_of("log-conf"));

    env_logger::init();
    log::set_max_level(log::LevelFilter::Debug);
    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let topic = "input-topic";
    let brokers = "localhost:29092";
    let msg =  vec![1; 1024];
    produce(brokers, topic, &msg).await;
}
