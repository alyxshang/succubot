/*
Discord Oxide by Alyx Shang.
Licensed under the FSL v1.
*/

use std::ops::DivAssign;
use std::sync::Arc;

use tokio::sync::Mutex;

use tokio::time::sleep;

use std::env::consts::OS;

use serde_json::from_str;

use tokio::net::TcpStream;

use tokio::time::Duration;

use futures_util::SinkExt;

use futures_util::StreamExt;

use super::units::IdentifyInfo;

/// Importing the structure for 
/// catching and handling errors.
use super::err::DiscordOxideErr;

use serde_json::to_string_pretty;

use super::units::HostProperties;

use super::units::IdentifyPayload;

use super::units::HeartBeatPayload;

use super::units::HandshakeMessage;

/// Importing the function to
/// establish an asynchronous
/// connection using "tokio".
use tokio_tungstenite::connect_async;

use tokio_tungstenite::MaybeTlsStream;

use tokio_tungstenite::WebSocketStream;

use tokio_tungstenite::tungstenite::Message;

use tokio_tungstenite::tungstenite::Utf8Bytes;

use tokio_tungstenite::tungstenite::client::IntoClientRequest;

pub fn generate_identify_payload(
    token: &str,
    name: &str,
    intents: &u64,
) -> Result<String, DiscordOxideErr>{
    let props: HostProperties = HostProperties{
        os: OS.to_string(),
        browser: name.to_string(),
        device: name.to_string(),
    };
    let id_info: IdentifyInfo = IdentifyInfo{
        token: token.to_string(),
        intents: *intents,
        host_properties: props
    };
    let pl: IdentifyPayload = IdentifyPayload{
        op: 2,
        d: id_info
    };
    let s: String = match to_string_pretty(&pl){
        Ok(s) => s,
        Err(e) => return Err::<String, DiscordOxideErr>(
            DiscordOxideErr::new(&e.to_string())
        )
    };
    Ok(s)
}

pub async fn retrieve_heartbeat_interval(
    websocket_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>
) -> Result<u64, DiscordOxideErr>{
    let hello_msg = match websocket_stream.next().await{
        Some(hello_msg) => hello_msg,
        None => return Err::<u64, DiscordOxideErr>(
            DiscordOxideErr::new("Handshake not received.")
        )
    };
    let msg = match hello_msg{
        Ok(msg) => msg,
        Err(e) => return Err::<u64, DiscordOxideErr>(
            DiscordOxideErr::new("Unable to parse handshake message.")
        )
    };
    let msg_str: String = match msg.to_text(){
        Ok(msg_str) => msg_str.to_string(),
        Err(e) => return Err::<u64, DiscordOxideErr>(
           DiscordOxideErr::new(&e.to_string())
        )
    };
    let deserialized: HandshakeMessage = match from_str(&msg_str){
        Ok(deserialized) => deserialized,
        Err(e) => return Err::<u64, DiscordOxideErr>(
            DiscordOxideErr::new(&e.to_string())
        )
    };
    Ok(deserialized.d.heartbeat_interval)
}

pub async fn establish_gateway(
    token: &str,
    name: &str,
    intents: &u64
) -> Result<(), DiscordOxideErr>{
    let gw_url: String = "wss://gateway.discord.gg/?v=10&encoding=json"
        .to_string();
    let client_req = match gw_url.into_client_request(){
        Ok(client_req) => client_req,
        Err(e) => return Err::<(), DiscordOxideErr>(
            DiscordOxideErr::new(&e.to_string())
        )
    };
    let (mut ws_stream, response) = match connect_async(client_req).await {
        Ok((ws_stream, response)) => (ws_stream, response),
        Err(e) => return Err::<(), DiscordOxideErr>(
            DiscordOxideErr::new(&e.to_string())
        )
    };
    println!("Connected to gateway");
    let sequence = Arc::new(Mutex::new(None::<u64>));
    let interval: u64 = match retrieve_heartbeat_interval(&mut ws_stream).await {
        Ok(interval) => interval,
        Err(e) => return Err::<(), DiscordOxideErr>(
            DiscordOxideErr::new(&e.to_string())
        )
    };
    println!("{:?}", interval);
    let ws_arc = Arc::new(Mutex::new(ws_stream));
    let hb: () = match send_heartbeat(
        interval, 
        sequence, 
        ws_arc
    ).await {
        Ok(_f) => {},
        Err(e) => return Err::<(), DiscordOxideErr>(
            DiscordOxideErr::new(&e.to_string())
        )
    };
    println!("Heartbeats sending.");
    let id_payload: String = match generate_identify_payload(
        token, 
        name, 
        intents
    ){
        Ok(id_payload) => id_payload,
        Err(e) => return Err::<(), DiscordOxideErr>(
            DiscordOxideErr::new(&e.to_string())
        )
    };
    let send_identify: () = match send_identify(
        &id_payload, 
        ws_stream
    ).await {
        Ok(_send_identify) => {},
        Err(e) => return Err::<(), DiscordOxideErr>(
            DiscordOxideErr::new(&e.to_string())
        )
    };
    while let Some(msg) = ws_stream.next().await {
        let msg: Message = match msg{
            Ok(msg) => msg,
            Err(e) => return Err::<(), DiscordOxideErr>(
                DiscordOxideErr::new(&e.to_string())
            )
        };
        println!("{:?}", msg);
    }
    Ok(())
}

pub async fn send_heartbeat(
    heartbeat: u64,
    seq: Arc<Mutex<Option<u64>>>,
    websocket_stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>
) -> Result<(), DiscordOxideErr>{
    tokio::spawn(
        async move {
            loop{
                let count = {
                    let sq = seq.lock().await; // async lock
                    match *sq {
                        Some(count) => count,
                        None => return Err(DiscordOxideErr::new("No integer found.")),
                    }
                };
                let mut ws_stream = websocket_stream.lock().await;
                let pl: HeartBeatPayload = HeartBeatPayload{
                    op: 1,
                    d: count
                };
                let payload: String = match to_string_pretty(
                    &pl
                ) {
                    Ok(payload) => payload,
                    Err(e) => return Err::<(), DiscordOxideErr>(
                        DiscordOxideErr::new(&e.to_string())
                    )
                };
                let bytes: Utf8Bytes = Utf8Bytes::from(payload);
                let send: () = match ws_stream.send(
                    Message::Text(bytes)
                ).await {
                    Ok(feedback) => {},
                    Err(e) => return Err::<(), DiscordOxideErr>(
                        DiscordOxideErr::new(&e.to_string())
                    )
                };
                sleep(Duration::from_millis(heartbeat)).await;
            }
        }
    );
    Ok(())
}

pub async fn send_identify(
    payload: &String,
    websocket_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>
) -> Result<(), DiscordOxideErr>{
    let bytes: Utf8Bytes = Utf8Bytes::from(payload);
    let send: () = match websocket_stream.send(Message::Text(bytes)).await {
        Ok(_send) => {},
        Err(e) => return Err::<(), DiscordOxideErr>(
            DiscordOxideErr::new(&e.to_string())
        )
    };
    Ok(send)
}
