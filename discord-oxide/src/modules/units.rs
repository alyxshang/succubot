/*
Discord Oxide by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the 
/// "Serialize" macro to
/// encode a Rust structure
/// into a JSON string.
use serde::Serialize;

use serde::Deserialize;

/// An enum to store
/// all possible types
/// of commands received.
#[derive(Clone)]
pub enum CommandType{
    NakedWord,
    SlashCommand
}

/// A structure to
/// hold information
/// on a command received.
pub struct Command {
    pub name: String,
    pub command_type: CommandType,
    pub verbs: Option<Vec<String>>,
    pub users: Option<Vec<String>>
}

/// A structure to send
/// a response to an interaction
/// inside a Discord server.
#[derive(Serialize)]
pub struct ReplyPayload {
    pub content: String
}

/// A structure to retrieve 
/// handshake info from the Discord
/// websocket stream.
#[derive(Deserialize)]
pub struct HandshakeMessage {
    pub t: Option<String>,
    pub s: Option<String>,
    pub op: u64,
    pub d: HeartBeatInfo
}

/// A structure to heartbeat
/// info from the Discord
/// websocket stream.
#[derive(Deserialize)]
pub struct HeartBeatInfo{
    pub heartbeat_interval: u64,
}

/// A structure to hold information
/// on the bot and where it is being run.
#[derive(Serialize)]
pub struct IdentifyPayload{
    pub op: u64,
    pub d: IdentifyInfo
}

/// A structure to hold information
/// on authentication information
/// of a bot.
#[derive(Serialize)]
pub struct IdentifyInfo {
    pub token: String,
    pub intents: u64,
    #[serde(rename(serialize = "properties"))]
    pub host_properties: HostProperties
}

/// A structure to hold meta information
/// on a bot, including operating system
/// information.
#[derive(Serialize)]
pub struct HostProperties{
    #[serde(rename(serialize = "$os"))]
    pub os: String,
    #[serde(rename(serialize = "$browser"))]
    pub browser: String,
    #[serde(rename(serialize = "$device"))]
    pub device: String
}

/// A structure to hold info
/// on the current heartbeat cycle.
#[derive(Serialize)]
pub struct HeartBeatPayload {
    pub op: u64,
    pub d: u64
}
