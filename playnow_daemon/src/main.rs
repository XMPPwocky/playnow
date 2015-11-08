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
extern crate playnow_core;

use std::thread;

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

fn bravely_get_env(name: &str) -> String {
    match std::env::var(name) {
        Ok(value) => value,
        _ => panic!("Environment variable {} not set.", name)
    }
}

fn get_apikey() -> String { bravely_get_env("STEAM_APIKEY") }
fn get_postgres_url() -> String { bravely_get_env("POSTGRES_URL") }
fn get_redis_url() -> String { bravely_get_env("REDIS_URL") }

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
