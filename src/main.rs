#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rumqtt;

use std::thread;

mod config;
mod mqtt;
mod cli;

// Test
// this documentation is a test
fn main() {
    // Reading configuration file.
    let config = config::read("./src/config.json");

    // Create a MqttContext by calling start.
    let mqtt_context = mqtt::MqttContext::start(config);

    // Clone the sender part of a communication channel, so that
    // we can send data to the MqttContext thread (subscribing to
    // topics, for instance)
    let tx_cli = mqtt_context.clone_sender();

    println!("Starting CLI thread...");

    //
    // Starting the CLI thread.
    // We move to it:
    //  - the sender part of the communication channel.
    //
    // We have to unwrap it because the result of 'spawn' method is actually
    // an Option object.
    let cli_thread = thread::Builder::new().spawn(move || {
        // Create a new CliContext passing the communication endpoint to it.
        let cli_context = cli::CliContext::new(tx_cli);

        // Start it!
        cli_context.start();
    }).unwrap();

    println!("... CLI started.");

    // Joins.
    cli_thread.join().unwrap();
}
