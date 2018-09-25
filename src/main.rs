#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serenity;
extern crate toml;

use std::path::Path;
use std::io::ErrorKind;
use model::command::{Command, Result};
use model::config::Config;
use model::error::Error;
use serenity::client::{Client, EventHandler};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use std::env;

mod model;

const CMD_PREFIX: &str = "!";

struct Handler {
    commands: Vec<Command>,
}

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        let ref content = msg.content;
        if content.starts_with(&CMD_PREFIX) {
            let label = get_label(&msg);
            for cmd in self.commands.iter() {
                if cmd.label == label {
                    cmd.exec(&msg);
                }
            }
        }
    }
}

fn main() {
    // TODO Create commands list
    let commands = vec! [
        Command {
            label: "ping".into(),
            desc: "pong".into(),
            help: "Syntax: `ping`".into(),
            run: |msg| {
                msg.channel_id.say("pong").unwrap();
                Result::Ok
            }
        }
    ];

    let mut cd = env::current_dir().unwrap();
    cd.push("conf.toml");

    // Check for config, create default if not found
    load_config(&cd);

    // Start the client
    let mut client = Client::new(&env::var("D_TOKEN").expect("No token specified."), Handler { commands }).expect("Error creating client.");
    if let Err(why) = client.start() {
        println!("Error while running the client: {:?}", why);
    }
}

fn parse(msg: &Message) -> (&str, Vec<&str>) {
    let mut args: Vec<&str> = msg.content.split(" ").collect();
    (args.remove(0).trim_left_matches(&CMD_PREFIX), args)
}
fn get_label(msg: &Message) -> &str {
    let mut split = msg.content.splitn(2, " ");
    split.next().unwrap().trim_left_matches(&CMD_PREFIX) // this will never be called on an empty message so this shouldn't be able to fail
}

fn load_config(path: &Path) -> Config {
    let config: Config = match Config::load(path) {
        Ok(config) => { config }
        Err(why) => {
            match why {
                Error::Io(e) => {
                    match e.kind() {
                        ErrorKind::NotFound => {
                            println!("No config file found, creating default.");
                            let config: Config = Default::default();
                            save_config(path, &config);
                            return config
                        }
                        _ => {
                            panic!("Couldn't load config file, ensure it is readable.");
                        }
                    }
                }
                Error::TomlDeserialize(e) => {
                    panic!("Config is not valid TOML: {:?}", e);
                }
                _ => {
                    unreachable!();
                }
            };
        }

    };
    config
}

fn save_config(path: &Path, config: &Config) {
    match config.save(path) {
        Ok(_) => {},
        Err(why) => {
            match why {
                Error::Io(e) => {
                    println!("Error saving default config: {:?}", e);
                }
                Error::TomlSerialize(e) => {
                    panic!("Failed to serialize config: {:?}", e);
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }
}

fn start_vote(msg: &Message) {

}
