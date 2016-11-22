extern crate clap;
extern crate num_cpus;

use self::clap::{Arg, App, ArgMatches};
use std::str::FromStr;

#[derive(Debug)]
pub struct Configuration {
    pub port: usize,
    pub interface: String,
    pub num_threads: usize
}

pub fn new() -> Configuration {
    let configs = get_cmd_args();

    Configuration {
        num_threads: match configs.value_of("Threads") {
            Some(threads) => match usize::from_str(threads) {
                Ok(threads) => threads,
                Err(_) => panic!("Unable to parse the number of threads to use.")
            },
            None => num_cpus::get() as usize
        },
        port: match configs.value_of("Port") {
            Some(port) => match usize::from_str(port) {
                Ok(port) => port,
                Err(_) => panic!("Unable to parse the port number to bind to.")
            },
            None => 6771
        },
        interface: match configs.value_of("Interface") {
            Some(iface) => iface.to_string(),
            None => "127.0.0.1".to_string()
        }
    }
}

pub fn get_cmd_args<'a>() -> ArgMatches<'a> {
    App::new("rackety")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Alan Hoffmeister <alanhoffmeister@gmail.com>")
        .about("Blazing fast in-memory key/value storage")
        .arg(Arg::with_name("Port")
            .short("p")
            .long("port")
            .help("The port where the server will listen to. Defaults to 6771")
            .takes_value(true))
        .arg(Arg::with_name("Interface")
            .short("i")
            .long("interface")
            .help("The interface where the server will bind to. Defaults to 127.0.0.1")
            .takes_value(true))
        .arg(Arg::with_name("Threads")
            .short("t")
            .long("threads")
            .takes_value(true)
            .help("The number of threads to use. Defaults to the amount of cores in the machine"))
        .get_matches()
}
