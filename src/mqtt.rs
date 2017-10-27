use rumqtt::{MqttOptions, MqttCallback, MqttClient, QoS, Message};

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use config;
use std::str;

pub struct MqttContext {
    sender_channel: Sender<String>
}

fn counter_cb(message: Message) {
    println!("message --> {:?}", message);
    let str_payload = str::from_utf8(&message.payload).expect("error in message");
    println!("Payload: {}", str_payload);
}

impl MqttContext {
    pub fn start(cfg: config::Config) -> MqttContext {
        let client_options = MqttOptions::new()
                .set_keep_alive(cfg.keep_alive)
                .set_reconnect(cfg.reconnect)
                .set_broker(&format!("{}:{}", cfg.broker_address, cfg.broker_port));
        let message_cb = MqttCallback::new().on_message(counter_cb);
        let mut client = MqttClient::start(client_options, Some(message_cb)).expect("Coudn't start");

        // Prepare the channels
        let (tx_mqtt, rx_mqtt): (Sender<String>, Receiver<String>) = mpsc::channel();

        // We should use Arc and Mutex so that we can pass to the receiver
        // thread
        let rx_shared = Arc::new(Mutex::new(rx_mqtt));

        thread::Builder::new().spawn(move || {
            let rx = rx_shared.lock().unwrap();
            loop {
                let topic = rx.recv().unwrap();
                let topics = vec![(topic.as_str(), QoS::Level0)];
                println!("Subscribing to topic: {:?}", topics);
                client.subscribe(topics).expect("Failure in subscription");
            }
        }).unwrap();

        MqttContext {
            sender_channel: tx_mqtt
        }

    }

    pub fn clone_sender(&self) -> Sender<String> {
        self.sender_channel.clone()
    }
}