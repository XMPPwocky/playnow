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
extern crate time;

pub mod backend;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum QueueStatus {
    NotQueuing,
    Queuing(Vec<GameServerId>),
    MatchFound(u64),
    MatchConfirmed(GameServerId)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct GameServerId(pub u32);

impl redis::FromRedisValue for GameServerId {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<GameServerId> {
        use std::convert::From;
        use std::str::FromStr;
        use redis::Value::*;

        match *v {
            Int(i) => {
                Ok(GameServerId(i as u32))
            },
            Data(ref data) => {
                let string = std::str::from_utf8(data).ok();
                let number = string.and_then(|string| u32::from_str(string).ok());

                match number {
                    Some(number) => {
                        Ok(GameServerId(number))
                    },
                    None => {
                        Err(From::from((redis::ErrorKind::TypeError, "Not numeric")))
                    }
                }
            },
            _ => {
                Err(From::from((redis::ErrorKind::TypeError, "Not numeric")))
            }
        }
    }
}

impl redis::ToRedisArgs for GameServerId {
    fn to_redis_args(&self) -> Vec<Vec<u8>> {
        let s = self.0.to_string();
        vec![s.as_bytes().to_vec()]
    }
}

