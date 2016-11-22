use ::coio::net::{TcpStream};
use ::protocol;
use std::io::{BufReader, BufRead};
use std::io::{Write};

pub enum Error {
    ClientCreationError,
    ReadLineError,
    SocketWriteError
}

pub struct Client {
    socket: TcpStream,
    reader: BufReader<TcpStream>
}

pub trait ClientResponse {
    fn format(&self) -> Vec<u8>;
}

impl Client {
    pub fn read_line(&mut self) -> Result<String, Error> {
        let mut line = String::new();

        match self.reader.read_line(&mut line) {
            Ok(_) => Ok(line),
            Err(_) => Err(Error::ReadLineError)
        }
    }

    pub fn read_command(&mut self) -> Result<protocol::Command, Error> {
        match self.read_line() {
            Ok(line) => Ok(protocol::parse(&line)),
            Err(e) => Err(e)
        }
    }

    pub fn reply<T: ClientResponse>(&mut self, response: &T) -> Result<(), Error> {
        let payload = response.format();

        match self.socket.write_all(&payload) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::SocketWriteError)
        }
    }
}

pub fn from_tcp_stream(socket: &TcpStream) -> Result<Client, Error> {
    let socket = match socket.try_clone() {
        Ok(socket) => socket,
        Err(_) => return Err(Error::ClientCreationError)
    };

    let reader = match socket.try_clone() {
        Ok(socket) => BufReader::new(socket),
        Err(_) => return Err(Error::ClientCreationError)
    };

    Ok(Client {
        socket: socket,
        reader: reader
    })
}
