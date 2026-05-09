use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Command {
    Stats,
    Errors,
    TopUsers,
}

impl Command {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "stats" => Ok(Self::Stats),
            "errors" => Ok(Self::Errors),
            "top-users" => Ok(Self::TopUsers),
            _ => Err(format!("Unknown command: {}", s)),
        }
    }
}
