/*
Discord Oxide by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Client"
/// structure from the "reqwest"
/// crate to make POST requests.
use reqwest::Client;

/// Importing the data 
/// structure for explicitly
/// typing the result of a request.
use reqwest::Response;

/// Importing the data structure
/// to help encode a reply to a
/// message into a JSON string.
use super::units::ReplyPayload;

/// Importing the structure for 
/// catching and handling errors.
use super::err::DiscordOxideErr;

/// Importing the function to
/// encode a Rust data structure
/// into a pretty JSON string.
use serde_json::to_string_pretty;

/// Importing the enum variant
/// for setting the "Content-Type"
/// header.
use reqwest::header::CONTENT_TYPE;

/// Importing the enum variant
/// for setting the "Authorization"
/// header.
use reqwest::header::AUTHORIZATION;

pub async fn send_reply(
    channel_id: &str,
    message: &str,
    token: &str
) -> Result<bool, DiscordOxideErr>{
    let url = format!(
        "https://discord.com/api/v10/channels/{}/messages", 
        channel_id
    );
    let reply_pl: ReplyPayload = ReplyPayload{
        content: message.to_string()
    };
    let reply_json: String = match to_string_pretty(
        &reply_pl
    ){
        Ok(reply_json) => reply_json,
        Err(e) => return Err::<bool, DiscordOxideErr>(
            DiscordOxideErr::new(&e.to_string())
        )
    };
    let client: Client = Client::new();
    let response: Response = match client
        .post(&url)
        .header(AUTHORIZATION, token)
        .header(CONTENT_TYPE, "application/json")
        .body(reply_json)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => return Err::<bool, DiscordOxideErr>(
            DiscordOxideErr::new(&e.to_string())
        )
    };
    Ok(response.status().is_success())
}
