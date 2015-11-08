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

pub mod backend;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum QueueStatus {
    NotQueuing,
    Queuing,
    MatchFound(u64),
    MatchConfirmed(GameServerId)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct GameServerId(pub u32);
