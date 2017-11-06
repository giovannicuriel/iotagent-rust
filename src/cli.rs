use std::io;
use std::sync::mpsc::Sender;


// Tokenize a string
fn tokenize(input: &String, separator: char) -> Result<Vec<String>, &'static str> {
    let mut ret: Vec<String> = Vec::new();
    let mut temp_token = String::new();
    for c in input.chars() {
        // Unfortunately, there is no way to write:
        // match c {
        //  separator =>  do_something()
        // }
        match c {
            k if (k == separator) => {
                ret.push(temp_token);
                temp_token = String::new();
            },
            k if (k == '\n') => {
                // Do nothing.
            },
            _ => {
                temp_token.push(c);
            }
        }
    }
    // Pushing last token
    ret.push(temp_token);
    Ok(ret)
}

// A CliContext
// Responsible for everything related to the internal CLI.
pub struct CliContext {
    subscr_channel: Sender<String>
}

impl CliContext {
    // Creates a new object
    pub fn new(subscr_channel: Sender<String>) -> CliContext {
        CliContext { subscr_channel: subscr_channel}
    }

    pub fn start(&self) {
        let mut input = String::new();
        println!("Subscribing to the default /test topic...");
        self.subscr_channel.send("/test".into()).expect("Failure while sending command to thread");
        loop {
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let tokens: Vec<String> = tokenize(&input, ' ').expect("Failure while tokeninzing");
                    match self.process_input(tokens) {
                        Err(msg) => {
                            println!("Error: {}", msg);
                        },
                        _ => {
                            println!("Command processed successfully");
                        }
                    }
                    input.clear();
                }
                Err(error) => println!("error: {}", error),
            }
        }
    }

    fn process_input(&self, input: Vec<String>) -> Result<bool, &'static str> {
        match input[0].as_str() {
            "help" => {
                println!("There is not much that you can do right now");
            },
            "subscribe" => {
                return self.process_subscription(&input[1..]);
            },
            _ => {
                return Err("Command not registered yet.");
            }
        }
        Ok(true)
    }

    fn process_subscription(&self, input: &[String]) -> Result<bool, &'static str> {
        match input[0].as_str() {
            "new" => {
                self.subscr_channel.send(String::clone(&input[1])).expect("Failure while sending command to thread");
            },
            _ => {
                return Err("Invalid operator for subscriptions.");
            }
        }
        Ok(true)
    }
}