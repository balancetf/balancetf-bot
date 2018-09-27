
use std::io::Error as IoError;
use serenity::Error as DiscordError;
use toml::ser::Error as SerError;
use toml::de::Error as DeError;

/// Errors that can occur while the bot is running.
#[derive(Debug)]
pub enum Error {
    /// An I/O operation failed.
    Io(IoError),
    /// Errors caused by the Discord API.
    Discord(DiscordError),
    /// An error occured when trying to serialize a type.
    TomlSerialize(SerError),
    /// An error occured when trying to deserialize a type.
    TomlDeserialize(DeError),
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Error::Io(error)
    }
}

impl From<DiscordError> for Error {
    fn from(error: DiscordError) -> Self {
        Error::Discord(error)
    }
}   

impl From<SerError> for Error {
    fn from(error: SerError) -> Self {
        Error::TomlSerialize(error)
    }
}

impl From<DeError> for Error {
    fn from(error: DeError) -> Self {
        Error::TomlDeserialize(error)
    }
}

