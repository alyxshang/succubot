/*
Discord Oxide by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// for encapsulating the 
/// information from a parsed,
/// received command.
use super::units::Command;

/// Importing the enum that
/// lists out the types of
/// commands a bot can receive.
use super::units::CommandType;

/// Importing the structure for 
/// catching and handling errors.
use super::err::DiscordOxideErr;

pub fn parse_command(
    msg: &str
) -> Result<Command, DiscordOxideErr>{
    let binding: String = msg.to_string();
    let mut slices: Vec<&str> = binding
        .split(" ")
        .collect::<Vec<&str>>();
    if slices.len() >= 1{
        let first: String = slices[0].to_string();
        let mut first_chars: Vec<char> = first
            .chars()
            .collect::<Vec<char>>();
        let command_type: CommandType;
        let name: String;
        if first_chars[0] == '/'{
            command_type = CommandType::SlashCommand;
            first_chars.remove(0);
            name = first_chars
                .iter()
                .collect::<String>();
            
        }
        else {
            command_type = CommandType::SlashCommand;
            name = first_chars
                .iter()
                .collect::<String>();
        }
        if slices.len() == 1 {
            let command: Command = Command{
                name: name.to_string(),
                command_type: command_type,
                verbs: None,
                users: None
            };
            Ok(command)
        }
        else {
            slices.remove(0);
            let users: Option<Vec<String>>;
            let verbs: Option<Vec<String>>;
            let mut user_vec: Vec<String> = Vec::new();
            let mut verb_vec: Vec<String> = Vec::new();
            for cmd_slice in slices {
                if is_username(cmd_slice){
                    let uid: String = match parse_username(cmd_slice){
                        Ok(uid) => uid,
                        Err(e) => return Err::<Command, DiscordOxideErr>(
                            DiscordOxideErr::new(&e.to_string())
                        )
                    };
                    user_vec.push(uid);
                }
                else {
                    verb_vec.push(cmd_slice.to_string());
                }
            }
            if user_vec.len() == 0{
                users = None;
            }
            else {
                users = Some(user_vec);
            }
            if verb_vec.len() == 0{
                verbs = None;
            }
            else {
                verbs = Some(verb_vec);
            }
            let command: Command = Command{
                name: name.to_string(),
                command_type: command_type,
                verbs: None,
                users: None
            };
            Ok(command)
        }
    }
    else {
        Err::<Command, DiscordOxideErr>(
            DiscordOxideErr::new("Invalid command received.")
        )
    }
}

pub fn is_numeric(
    slice: &str
) -> bool {
    let mut result: bool = true;
    let alphabet: Vec<char> = "1234567890"
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    let slice_chars: Vec<char> = slice
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    for character in slice_chars {
        if !alphabet.contains(&character){
            result = false;
        }
    }
    result
}

pub fn is_username(
    slice: &str
) -> bool {
    let mut result: bool = true;
    let mut slice_chars: Vec<char> = slice
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    let last_idx: usize = slice_chars.len() - 1;
    if slice_chars[0] == '<' && 
       slice_chars[1] == '@' &&
       slice_chars[last_idx] == '>'
    {
        if slice_chars.len() >= 17 {
            slice_chars.remove(0);
            slice_chars.remove(0);
            slice_chars.remove(slice_chars.len() - 1);
            let uid: String = slice_chars
                .iter()
                .collect::<String>();
            if is_numeric(&uid){
                result = true;
            }
            else {
                result = false;
            }
        }
        else {
            result = false;
        }
    }
    else {
        result = false;
    }
    result
}

pub fn parse_username(
    slice: &str
) -> Result<String, DiscordOxideErr>{
    let mut slice_chars: Vec<char> = slice
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    let last_idx: usize = slice_chars.len() - 1;
    if is_username(slice){
        slice_chars.remove(0);
        slice_chars.remove(0);
        slice_chars.remove(slice_chars.len() - 1);
        let uid: String = slice_chars
            .iter()
            .collect::<String>();
        Ok(uid)
    }
    else {
        let e: String = format!(
            "\"{}\" is not a username.",
            slice
        );
        Err::<String, DiscordOxideErr>(
            DiscordOxideErr::new(&e.to_string())
        )
            
    }
}
