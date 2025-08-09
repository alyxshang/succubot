/*
Discord Oxide by Alyx Shang.
Licensed under the FSL v1.
*/

use super::err::DiscordOxideErr;

pub enum Intent{
}

pub struct BotCommand{
    pub name: String,
    pub description: String,
    pub arguments: u64
}

impl BotCommand {

    pub fn new(
        name: &str,
        description: &str,
        arguments: &u64
    ) -> BotCommand{
        BotCommand{
            name: name.to_string(),
            description: description.to_string(),
            arguments: *arguments
        }
    }
}

pub struct Bot{
    pub name: String,
    pub token: String,
    pub commands: Vec<BotCommand>,
    pub intents: Vec<Intent>
}

impl Bot {
    pub fn new(
        name: &str,
        token: &str,
        intents: &Vec<Intent>
    ) -> Bot {
        let commands: Vec<BotCommand> = Vec::new();
        Bot {
            name: name.to_string(),
            token: token.to_string(),
            commands: commands,
            intents: *intents
        }
    }

    pub fn add_command(
        &self,
        name: &str,
        description: &str,
        arguments: &u64,
    ) -> () {
        let cmd: BotCommand = BotCommand::new(
            name,
            description,
            arguments
        );
        &self.push(cmd);
    }
}
