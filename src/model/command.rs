use super::error::Error;
use serenity::model::channel::Message;

/// A representation of a command that users can type in chat.
pub struct Command {
    /// The label of the command. This is what users type to invoke the command.
    /// For example, if `label` is `help`, then "!help" would be used to invoke it, assuming cmd
    /// prefix is !.
    pub label: String,
    /// The description of the command. Just a short summary of what it does and its syntax.
    pub desc: String,
    /// A more thorough explanation of the command and how to use it. This is shown when a user
    /// incorrectly uses a command.
    pub help: String,
    /// The permission a role must have to execute the command.
    pub perm: String,
    /// The function that gets run when the command is invoked.
    pub run: fn(&Message) -> Result,
}

impl Command {
    /// Calls the function associated with this command, with `msg` as an argument.
    pub fn exec(&self, msg: &Message) {
        match (self.run)(msg) {
            Result::Ok => {}
            Result::Syntax => match msg.channel_id.say(&self.help) {
                Ok(_) => {}
                Err(why) => {
                    println!(
                        "Error sending help message for command:{}\nReason:\n{}",
                        &self.label, why
                    );
                }
            },
            Result::InvalidArg(explain) => match msg.channel_id.say(&explain) {
                Ok(_) => {}
                Err(why) => {
                    println!(
                        "Error sending help message for command:{}\nReason:\n{}",
                        &self.label, why
                    );
                }
            },
            Result::Error(why) => {
                println!(
                    "Error when running command: {}\nReason:\n{:?}",
                    &self.label, why
                );
            }
        }
    }
}

/// The possible outcomes of a user invoking a command.
pub enum Result {
    /// The command executed successfully with no problems.
    Ok,
    /// The user invoked the command with invalid syntax.
    Syntax,
    /// The user made an error when running the command.
    InvalidArg(String),
    /// An error occured that wasn't the user's fault.
    Error(Error),
}

// When commands are added that can error, a custom error type should be implemented here, with an
// error type enum.
