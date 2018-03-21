// use rumqtt::{MqttOptions, MqttCallback, MqttClient, QoS, Message};

// use std::thread;
// use std::sync::mpsc;
// use std::sync::mpsc::{Receiver, Sender};
// use std::sync::Arc;
// use std::sync::Mutex;
// use serde_json;
// use serde_json::{Value, Map};
// use config;
// use std::str;

// pub struct MqttContext {
//     sender_channel: Sender<String>
// }

// impl MqttContext {

//     fn flatten_and_send(message: Message) {
//         let str_payload = str::from_utf8(&message.payload).expect("error in message");
//         let parsed_data: Map<String, Value> = serde_json::from_str(str_payload).expect("Error while parsing JSON");

//         let results = MqttContext::flatten_object("".into(), parsed_data);
//         println!("Results: {:?}", results);
//     }

//     pub fn start(cfg: config::Config) -> MqttContext {
//         let client_options = MqttOptions::new()
//                 .set_keep_alive(cfg.keep_alive)
//                 .set_reconnect(cfg.reconnect)
//                 .set_broker(&format!("{}:{}", cfg.broker_address, cfg.broker_port));
//         let message_cb = MqttCallback::new().on_message(MqttContext::flatten_and_send);
//         let mut client = MqttClient::start(client_options, Some(message_cb)).expect("Coudn't start");

//         // Prepare the channels
//         let (tx_mqtt, rx_mqtt): (Sender<String>, Receiver<String>) = mpsc::channel();

//         // We should use Arc and Mutex so that we can pass to the receiver
//         // thread. This is because rx_mqtt doesn't implement a few traits to make
//         // that possible.
//         let rx_shared = Arc::new(Mutex::new(rx_mqtt));

//         thread::Builder::new().spawn(move || {
//             let rx = rx_shared.lock().unwrap();
//             loop {
//                 let topic = rx.recv().unwrap();
//                 let topics = vec![(topic.as_str(), QoS::Level0)];
//                 println!("Subscribing to topic: {:?}", topics);
//                 client.subscribe(topics).expect("Failure in subscription");
//             }
//         }).unwrap();

//         MqttContext {
//             sender_channel: tx_mqtt
//         }

//     }

//     pub fn clone_sender(&self) -> Sender<String> {
//         self.sender_channel.clone()
//     }
// }