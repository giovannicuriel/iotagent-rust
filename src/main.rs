#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rumqtt;

mod config;
mod mqtt;

fn main() {
    let config = config::read("./src/config.json");
    println!("This is the config: {:?}", config);
    let context = mqtt::init(config);
    mqtt::start(context);
}
