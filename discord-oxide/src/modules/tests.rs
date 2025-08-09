use super::cmd::is_numeric;
use super::cmd::is_username;
use super::cmd::parse_username;
#[test]
pub fn test_parse_username() -> () {
    match parse_username(
        "<@1327025357500448860>"
    ){
        Ok(parsed) => assert_eq!(
            String::from("1327025357500448860"),
            parsed
        ),
        Err(e) => eprintln!("{}", e)
    }
}
#[test]
pub fn test_is_numeric() -> () {
    assert_eq!(is_numeric("1327025357500448860"), true);
}
#[test]
pub fn test_is_username() -> () {
    assert_eq!(
        is_username(
            "<@1327025357500448860>"
        ),
        true
    );
}
