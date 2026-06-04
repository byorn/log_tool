use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Command {
    Stats,
    Errors,
    TopUsers,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stats" => Ok(Self::Stats),
            "errors" => Ok(Self::Errors),
            "top-users" => Ok(Self::TopUsers),
            _ => Err(format!("Unknown command: {}", s)),
        }
    }
}
