#![allow(dead_code)]
extern crate serde;
extern crate serde_json;
extern crate steamwebapi;
extern crate hyper;
extern crate redis;
extern crate websocket;
extern crate steamid;
#[macro_use]
extern crate quick_error;
extern crate postgres;

use std::thread;

mod backend;
mod wshandler;

#[derive(Clone, PartialEq, Eq, Debug)]
enum QueueStatus {
    NotQueuing,
    Queuing,
    MatchFound(u64),
    MatchConfirmed(GameServerId)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct GameServerId(u32);

fn get_apikey() -> String {
    match std::env::var("STEAM_APIKEY") {
        Ok(key) => key,
        _ => panic!("No Steam API key found. Set the STEAM_APIKEY environment variable.")
    }
}
fn get_postgres_url() -> String {
    match std::env::var("POSTGRES_URL") {
        Ok(key) => key,
        _ => panic!("No Postgres URI found. Set the POSTGRES_URL environment variable.")
    }
}

fn main() {
    let ws_server = websocket::Server::bind("127.0.0.1:2794").unwrap();
    //let mut backend = backend::Backend::new();

    for connection in ws_server {
        if let Ok(connection) = connection {
            thread::spawn(move || {
                wshandler::handler(connection) 
            });
        } else if let Err(e) = connection {
            println!("Error: {:?}", e);
            break;
        }
    }
}
