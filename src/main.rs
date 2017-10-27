#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rumqtt;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

mod config;
mod mqtt;
mod cli;


fn main() {

    let config = config::read("./src/config.json");
    println!("This is the config: {:?}", config);
    let mqtt_context = mqtt::MqttContext::start(config);

    let tx_cli = mqtt_context.clone_sender();

    println!("Starting CLI thread...");
    let cli_thread = thread::Builder::new().spawn(move || {
        let cli_context = cli::CliContext::new(tx_cli);
        cli_context.start();
    }).unwrap();
    println!("... CLI started.");

    // Joins.
    cli_thread.join().unwrap();
}
