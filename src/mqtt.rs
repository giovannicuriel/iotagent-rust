use rumqtt::{MqttOptions, MqttCallback, MqttClient, QoS, Message};

use std::thread;
use std::time::Duration;
use config;
use std::str;

pub struct MqttContext {
    client_options: MqttOptions,
    message_cb: MqttCallback,
}

fn counter_cb(message: Message) {
    println!("message --> {:?}", message);
    let str_payload = str::from_utf8(&message.payload).expect("error in message");
    println!("Payload: {}", str_payload);
}

pub fn init(cfg: config::Config) -> MqttContext {
    // Specify client connection options
    let client_options = MqttOptions::new()
        .set_keep_alive(cfg.keep_alive)
        .set_reconnect(cfg.reconnect)
        .set_broker(&format!("{}:{}", cfg.broker_address, cfg.broker_port));
    let msg_callback = MqttCallback::new().on_message(counter_cb);
    MqttContext {
        client_options: client_options,
        message_cb: msg_callback,
    }
}


pub fn start(context: MqttContext) {
    // Create a new `MqttClient` object from `MqttOptions`
    let mut mq_client = MqttClient::start(context.client_options, Some(context.message_cb))
        .expect("Coudn't start");

    let topics = vec![("test/basic", QoS::Level0)];
    mq_client.subscribe(topics).expect("Subcription failure");

    thread::sleep(Duration::new(5, 0));
}