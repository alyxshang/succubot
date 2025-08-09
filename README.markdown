# SUCCUBOT

```TOML
# Cargo.toml
[package]
name = "discord-oxide"
version = "0.1.0"
edition = "2024"

[dependencies]
tokio = { version = "*", features = ["full"] }
discord-oxide = { git = "https://github.com/alyxshang/succubot", tag = "v.0.1.0", path = "discord-oxide" }
```

```Rust
// main.rs
use tokio;
use discord_oxide::Bot;
use discord_oxide::Intent;
use discord_oxide::send_reply;
use discord_oxide::CommandType;
use discord_oxide::Interaction;

#[tokio::main]
async fn main() -> (){
  let my_bot: Bot = Bot::new(
    "My awesome bot",
    "MY_TOKEN_HERE",
    &vec![
      Intent::Guild
    ]
  );
  my_bot.add_command(
    "hello",
    CommandType::Slash,
    "greets the user",
    &0
  );
  my_bot.run(
    |interaction: Interaction|{
      if interaction.name == "hello"{
        send_reply("Hello yourself.");
      }
    }
  ).await;
}
```
