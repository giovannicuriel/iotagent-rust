#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rumqtt;
extern crate kafka;

use std::env;

mod config;
mod orion_handler;
mod data_broker;
mod device_manager_handler;
mod kafka_manager;

// use orion_handler::OrionHandler;

fn process_data(data: &[u8]) {
    println!("Data:");
    for d in data {
        println!("[{}]: {}", d, *d as char);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: node {} CONFIG_FILE.json", args[1]);
        return;
    }

    // Reading configuration file.
    let configuration = config::read_config("./config.json");

    let kafka_manager = kafka_manager::KafkaManager::new();
    kafka_manager.subscribe("teste".to_string(), process_data);

    // let orionHandler = OrionHandler::new(configuration);
    // let deviceManagerHandler = DeviceManagerHandler::new(configuration);
    // let agent = Agent::new(configuration, orionHandler, deviceManagerHandler);
    // agent.start_mqtt();


    // // Create a MqttContext by calling start.
    // let mqtt_context = mqtt::MqttContext::start(configuration);

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

    // //Send one message

    // let broker = "127.0.0.1:2181";
    // let topic = "subscription-xyz";

    // let data = "hello, kafka".as_bytes();

    // if let Err(e) = kafka_fn::produce_message(data, topic, vec![broker.to_owned()]) {
    //     println!("Failed producing messages: {}", e);
    // }


    // // Joins.
    // cli_thread.join().unwrap();
}
