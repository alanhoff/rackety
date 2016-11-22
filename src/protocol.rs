extern crate shlex;

use ::client::ClientResponse;

#[derive(Debug)]
pub enum Error {
    InvalidCommand
}

#[derive(Debug)]
pub enum Command {
    Set(SetCommand),
    Del(DelCommand),
    Get(GetCommand),
    Unknown
}

#[derive(Debug)]
pub struct SetCommand {
    pub key: String,
    pub value: String
}

pub struct Reply {
    data: Vec<u8>
}

impl ClientResponse for Reply {
    fn format(&self) -> Vec<u8> {
        self.data.clone()
    }
}

impl Reply {
    pub fn ok() -> Reply {
        Reply {
            data: "ok\r\n".as_bytes().to_vec()
        }
    }

    pub fn ok_with_data(data: &str) -> Reply {
        Reply {
            data: format!("ok {:?}\r\n", data).as_bytes().to_vec()
        }
    }

    pub fn error_key_not_found() -> Reply {
        Reply {
            data: "error key_not_found \"Unable to find the requested key\"\r\n".as_bytes().to_vec()
        }
    }

    pub fn error_unable_write_store() -> Reply {
        Reply {
            data: "error unable_write_store \"Unable to write to the store\"\r\n".as_bytes().to_vec()
        }
    }

    pub fn error_unknown_command() -> Reply {
        Reply {
            data: "error unknown_command \"The server didn't understood your command\"\r\n".as_bytes().to_vec()
        }
    }
}


impl SetCommand {
    pub fn new(args: &Vec<String>) -> Result<SetCommand, Error> {
        match args.len() {
            3 => match args.get(0) {
                Some(comand) if comand == "set" => {
                    Ok(SetCommand {
                        key: args.get(1).unwrap().to_owned(),
                        value: args.get(2).unwrap().to_owned()
                    })
                },
                Some(_) => Err(Error::InvalidCommand),
                None => Err(Error::InvalidCommand)
            },
            _ => Err(Error::InvalidCommand)
        }
    }
}

#[derive(Debug)]
pub struct DelCommand {
    pub key: String
}

impl DelCommand {
    pub fn new(args: &Vec<String>) -> Result<DelCommand, Error> {
        match args.len() {
            2 => match args.get(0) {
                Some(comand) if comand == "del" => {
                    Ok(DelCommand { key: args.get(1).unwrap().to_owned() })
                },
                Some(_) => Err(Error::InvalidCommand),
                None => Err(Error::InvalidCommand)
            },
            _ => Err(Error::InvalidCommand)
        }
    }
}

#[derive(Debug)]
pub struct GetCommand {
    pub key: String
}

impl GetCommand {
    pub fn new(args: &Vec<String>) -> Result<GetCommand, Error> {
        match args.len() {
            2 => match args.get(0) {
                Some(comand) if comand == "get" => {
                    Ok(GetCommand { key: args.get(1).unwrap().to_owned() })
                },
                Some(_) => Err(Error::InvalidCommand),
                None => Err(Error::InvalidCommand)
            },
            _ => Err(Error::InvalidCommand)
        }
    }
}

pub fn parse(str: &str) -> Command {
    let command = match shlex::split(&str.trim()) {
        Some(args) => args,
        None => return Command::Unknown
    };

    match command.get(0) {
        Some(cmd) => match cmd.as_str() {
            "del" => match DelCommand::new(&command) {
                Ok(del) => Command::Del(del),
                Err(_) => Command::Unknown
            },
            "get" => match GetCommand::new(&command) {
                Ok(get) => Command::Get(get),
                Err(_) => Command::Unknown
            },
            "set" => match SetCommand::new(&command) {
                Ok(set) => Command::Set(set),
                Err(_) => Command::Unknown
            },
            _ => Command::Unknown
        },
        None => Command::Unknown
    }
}
