use std::time::Duration;
use std::collections::HashMap;
use kafka::producer::{Producer, Record, RequiredAcks};
use kafka::error::Error as KafkaError;


pub fn produce_message<'a, 'b>(
    data: &'a [u8],
    topic: &'b str,
    brokers: Vec<String>,
) -> Result<(), KafkaError> {
    println!(
        "About to d publish a message at {:?} to: {}",
        brokers,
        topic
    );

    // ~ create a producer. this is a relatively costly operation, so
    // you'll do this typically once in your application and re-use
    // the instance many times.
    let mut producer = try!(
        Producer::from_hosts(brokers)
             // ~ give the brokers one second time to ack the message
             .with_ack_timeout(Duration::from_secs(1))
             // ~ require only one broker to ack the message
             .with_required_acks(RequiredAcks::One)
             // ~ build the producer with the above settings
             .create()
    );

    println!("oi");
    // ~ now send a single message.  this is a synchronous/blocking
    // operation.

    // ~ we're sending 'data' as a 'value'. there will be no key
    // associated with the sent message.

    // ~ we leave the partition "unspecified" - this is a negative
    // partition - which causes the producer to find out one on its
    // own using its underlying partitioner.
    // try!(producer.send(&Record {
    //     topic: topic,
    //     partition: -1,
    //     key: (),
    //     value: data,
    // }));

    // ~ we can achieve exactly the same as above in a shorter way with
    // the following call
    println!("Sending to topic: {:?} data {:?}", topic, data);
    try!(producer.send(&Record::from_value(topic, data)));

    Ok(())
}


type KafkaSubscriberCallback = fn(&[u8]);

pub struct KafkaManager {
    subscribers: HashMap<String, Vec<KafkaSubscriberCallback>>,
}

impl KafkaManager {
    pub fn new() -> KafkaManager {
        KafkaManager { subscribers: HashMap::new() }
    }

    pub fn subscribe(&self, topic: String, callback: KafkaSubscriberCallback) {
        let temp = "this is a test".as_bytes();
        callback(temp);
    }
}