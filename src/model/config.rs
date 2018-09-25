
use std::default::Default;
use std::io::{Read, Write};
use std::fs::OpenOptions;
use std::path::Path;
use super::error::Error;
use toml; // cargo spews errors without this line

/// A representation of the options that can be changed for the bot.
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// The prefix prepended to commands. Default is "!"
    pub cmd_prefix: String,
    /// The Discord role ID for keynote members.
    pub keynote_role: u64,
    /// The number of hours voting will be open when a new vote is created. Default is 24.
    pub vote_hours: u32,
    /// The Discord channel ID in which votes will be announced.
    pub channel_announce: u64,
    /// The Discord channel ID in which normal members can vote.
    pub channel_vote: u64,
    /// The Discord channel ID in which keynote members can vote.
    pub channel_keynote: u64,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            cmd_prefix: "!".into(),
            keynote_role: 0,
            vote_hours: 24,
            channel_announce: 0,
            channel_vote: 0,
            channel_keynote: 1, // this needs to be different from channel_vote
        }
    }
}

impl Config {
    /// Load a Config from file.
    pub fn load(path: &Path) -> Result<Config, Error> {
        // Open the file read-only. Errors if the file can't be opened.
        let mut file = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(path)?;
        // Read the file contents into a String. Errors if the file is not valid UTF-8.
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        // Parse the contents of the file into a Config. Errors if file contents are not valid
        // TOML.
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
    /// Save a Config to file.
    pub fn save(&self, path: &Path) -> Result<(), Error> {
        // Open file for writing. Errors if the file can't be opened.
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)?;
        // Serialize Config to TOML.
        let content = toml::to_string_pretty(self)?;
        // Write content to file.
        Ok(file.write_all(&content.as_bytes())?)
    }
}


