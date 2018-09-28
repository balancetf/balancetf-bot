#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serenity;
extern crate toml;

use model::command::{Command, Result};
use model::config::Config;
use model::error::Error;
use serenity::client::{Client, EventHandler};
use serenity::model::channel::Message;
use serenity::model::guild::Member;
use serenity::model::id::GuildId;
use serenity::prelude::Context;
use std::env;
use std::io::ErrorKind;
use std::path::Path;

mod model;

struct Handler {
    config: Config,
    commands: Vec<Command>,
}

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        let ref content = msg.content;
        if content.starts_with(&self.config.cmd_prefix) {
            let label = get_label(&msg, &self.config);
            let sender = match &msg.member {
                Some(m) => m,
                None => return, // early out if the message wasn't sent in a guild
            };
            for cmd in self.commands.iter() {
                if cmd.label == label && self.config.user_has_perm(&sender, &cmd.perm) {
                    cmd.exec(&msg);
                }
            }
        }
    }
    fn guild_member_addition(&self, _: Context, _: GuildId, mut member: Member) {
        match member.add_role(self.config.member_role) {
            Ok(_) => {}
            Err(why) => {
                println!("Error assigning member role to new member:\n{:?}", why);
            }
        };
    }
}

fn main() {
    // TODO Create commands list
    let commands = vec![Command {
        label: "ping".into(),
        desc: "pong".into(),
        help: "Syntax: `ping`".into(),
        perm: "btf.ping".into(),
        run: |msg| {
            msg.channel_id.say("pong").unwrap();
            Result::Ok
        },
    }];

    let mut cd = env::current_dir().unwrap();
    cd.push("conf.toml");

    // Check for config, create default if not found
    let config = load_config(&cd);

    // Start the client
    let mut client = Client::new(
        &env::var("D_TOKEN").expect("No token specified."),
        Handler { config, commands },
    )
    .expect("Error creating client.");
    if let Err(why) = client.start() {
        println!("Error while running the client: {:?}", why);
    }
}

fn get_args(msg: &Message) -> Vec<&str> {
    let mut args: Vec<&str> = msg.content.split(" ").collect();
    args.remove(0);
    args
}
fn get_label<'a, 'b>(msg: &'a Message, cfg: &'b Config) -> &'a str {
    let mut split = msg.content.splitn(2, " ");
    split.next().unwrap().trim_left_matches(&cfg.cmd_prefix) // this will never be called on an empty message so this shouldn't be able to fail
}

fn load_config(path: &Path) -> Config {
    let config: Config = match Config::load(path) {
        Ok(config) => config,
        Err(why) => {
            match why {
                Error::Io(e) => match e.kind() {
                    ErrorKind::NotFound => {
                        println!("No config file found, creating default.");
                        let config: Config = Default::default();
                        save_config(path, &config);
                        return config;
                    }
                    _ => {
                        panic!("Couldn't load config file, ensure it is readable.");
                    }
                },
                Error::TomlDeserialize(e) => {
                    panic!("Couldn't load config: {:?}", e);
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
        Ok(_) => {}
        Err(why) => match why {
            Error::Io(e) => {
                println!("Error saving default config: {:?}", e);
            }
            Error::TomlSerialize(e) => {
                panic!("Failed to serialize config: {:?}", e);
            }
            _ => {
                unreachable!();
            }
        },
    }
}
