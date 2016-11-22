#[macro_use]
extern crate slog;
extern crate coio;

#[macro_use]
extern crate log;

mod logger;
mod config;
mod store;
mod protocol;
mod client;

use coio::Scheduler;
use coio::net::{TcpListener};
use protocol::{Command, Reply};
use std::process::exit;

fn handle_client(client: &mut client::Client, store: &store::ArcStore) -> () {
    loop {
        match client.read_command() {
            Ok(command) => match command {
                Command::Set(set) => match store::safe_set(&store, set.key, set.value) {
                    Ok(_) => match client.reply(&Reply::ok()) {
                        Ok(_) => (),
                        Err(_) => break
                    },
                    Err(_) => match client.reply(&Reply::error_unable_write_store())  {
                        Ok(_) => (),
                        Err(_) => break
                    }
                },
                Command::Get(get) => match store::safe_get(&store, &get.key) {
                    Some(string) => match client.reply(&Reply::ok_with_data(&string))  {
                        Ok(_) => (),
                        Err(_) => break
                    },
                    None => match client.reply(&Reply::error_key_not_found())  {
                        Ok(_) => (),
                        Err(_) => break
                    }
                },
                Command::Del(del) => match store::safe_remove(&store, &del.key) {
                    Ok(()) => match client.reply(&Reply::ok())  {
                        Ok(_) => (),
                        Err(_) => break
                    },
                    Err(_) => match client.reply(&Reply::error_unable_write_store())  {
                        Ok(_) => (),
                        Err(_) => break
                    }
                },
                Command::Unknown => match client.reply(&Reply::error_unknown_command())  {
                    Ok(_) => (),
                    Err(_) => break
                }
            },
            Err(_) => break
        };
    }
}

fn main() {
    logger::setup();

    let config = config::new();
    let store = store::new();

    info!("Using {} threads", config.num_threads);
    let mut scheduler = Scheduler::new().with_workers(config.num_threads);
    let addr = format!("{}:{}", config.interface, config.port);

    loop {
        let addr = addr.clone();
        let store = store.clone();

        match scheduler.run(move|| {
            let acceptor = match TcpListener::bind(addr.as_str()) {
                Ok(acceptor) => acceptor,
                Err(_) => {
                    error!("Unable to bind the server to the address {}", addr);
                    exit(1);
                }
            };

            info!("Server started at {}", addr);
            for connection in acceptor.incoming() {
                let store = store.clone();
                let stream = match connection {
                    Ok((stream, _)) => stream,
                    Err(_) => break
                };

                let mut client = match client::from_tcp_stream(&stream) {
                    Ok(client) => client,
                    Err(_) => break
                };

                Scheduler::spawn(move|| handle_client(&mut client, &store));
            }
        }) {
            Ok(_) => break,
            Err(_) => continue
        };
    }
}
