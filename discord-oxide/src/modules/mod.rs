/*
Discord Oxide by Alyx Shang.
Licensed under the FSL v1.
*/

/// Exporting the module
/// containing the structure
/// to catch and handle errors.
pub mod err;

/// Exporting the module that
/// handles parsing any 
/// received messages.
pub mod cmd;

/// Exporting the module
/// containing this crate's
/// structures for encapsulating
/// data.
pub mod units;

/// Exporting the module
/// containing the function
/// to send a reply to an
/// interaction inside a
/// Discord server.
pub mod reply;

/// Exporting the structure
/// that establishes a gateway
/// with the Discord API to
/// receive events happening
/// on Discord.
pub mod gateway;

/// Declaring the module
/// for testing the code
/// in this crate.
#[cfg(test)]
pub mod tests;
