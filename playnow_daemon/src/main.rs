#![allow(dead_code)]

#[macro_use]
extern crate quick_error;
extern crate steamwebapi;
extern crate redis;
extern crate websocket;
extern crate steamid;
extern crate postgres;
extern crate playnow_core;
extern crate time;

use playnow_core::backend;

mod cron;

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
fn create_backend_pool() -> backend::BackendPool {
    backend::BackendPool::new(
        &bravely_get_env("STEAM_APIKEY"),
        &bravely_get_env("REDIS_URL"),
        &bravely_get_env("POSTGRES_URL")
        )
}

fn main() {
    std::thread::spawn(cron::cron_thread);
}
