/*
Discord Oxide by Alyx Shang.
Licensed under the FSL v1.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module
/// containing the structure
/// to catch and handle errors.
pub use modules::err::*;

/// Re-exporting the module that
/// handles parsing any 
/// received messages.
pub use modules::cmd::*;

/// Re-exporting the module
/// containing this crate's
/// structures for encapsulating
/// data.
pub use modules::units::*;

/// Re-exporting the module
/// containing the function
/// to send a reply to an
/// interaction inside a
/// Discord server.
pub use modules::reply::*;

/// Re-exporting the structure
/// that establishes a gateway
/// with the Discord API to
/// receive events happening
/// on Discord.
pub use modules::gateway::*;
