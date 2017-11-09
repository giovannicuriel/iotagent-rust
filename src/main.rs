#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rumqtt;
extern crate kafka;

use std::thread;

mod config;
mod mqtt;
mod cli;
mod kafka_fn;

// Test
// this documentation is a test
fn main() {
    // // Reading configuration file.
    // let config = config::read("./src/config.json");

    // // Create a MqttContext by calling start.
    // let mqtt_context = mqtt::MqttContext::start(config);

    // // Clone the sender part of a communication channel, so that
    // // we can send data to the MqttContext thread (subscribing to
    // // topics, for instance)
    // let tx_cli = mqtt_context.clone_sender();

    // println!("Starting CLI thread...");

    // //
    // // Starting the CLI thread.
    // // We move to it:
    // //  - the sender part of the communication channel.
    // //
    // // We have to unwrap it because the result of 'spawn' method is actually
    // // an Option object.
    // let cli_thread = thread::Builder::new().spawn(move || {
    //     // Create a new CliContext passing the communication endpoint to it.
    //     let cli_context = cli::CliContext::new(tx_cli);

    //     // Start it!
    //     cli_context.start();
    // }).unwrap();

    // println!("... CLI started.");

    //Send one message

    let broker: String = "127.0.0.1:2181".into();
    let topic = "subscription-xyz";

    let data = "hello, kafka".as_bytes();

    if let Err(e) = kafka_fn::produce_message(data, topic, vec![broker.to_owned()]) {
        println!("Failed producing messages: {}", e);
    }


    // Joins.
    // cli_thread.join().unwrap();
}
