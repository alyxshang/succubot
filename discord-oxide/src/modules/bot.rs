/*
Discord Oxide by Alyx Shang.
Licensed under the FSL v1.
*/

use super::units::CommandType;

#[derive(Clone)]
pub enum Intent{
}

#[derive(Clone)]
pub struct BotCommand{
    pub name: String,
    pub description: String,
    pub arguments: u64,
    pub command_type: CommandType
}

impl BotCommand {

    pub fn new(
        name: &str,
        description: &str,
        arguments: &u64,
        command_type: &CommandType
    ) -> BotCommand{
        BotCommand{
            name: name.to_string(),
            description: description.to_string(),
            arguments: *arguments,
            command_type: command_type.clone()
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
            intents: intents.to_vec()
        }
    }

    pub fn add_command(
        &mut self,
        name: &str,
        description: &str,
        arguments: &u64,
        command_type: &CommandType
    ) -> () {
        let cmd: BotCommand = BotCommand::new(
            name,
            description,
            arguments,
            command_type
        );
        &self.commands.push(cmd);
    }
}
