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
use playnow_core::create_backend_pool;

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

fn main() {
    std::thread::spawn(cron::cron_thread);
}
