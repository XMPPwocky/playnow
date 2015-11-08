use steamid::SteamId;
use steamwebapi;
use redis;
use redis::Commands;
use postgres;
use QueueStatus;
use GameServerId;

quick_error! {
    #[derive(Debug)]
    pub enum BackendError {
        Redis(err: redis::RedisError) {
            from()
            cause(err)
            description(err.description())
        }
        Postgres(err: postgres::error::DbError) {
            from()
            cause(err)
            description(err.description())
        }
    }
}
pub type BackendResult<T> = Result<T, BackendError>;

pub struct Backend {
    steam_webapi: steamwebapi::ApiClient,

    redis: redis::Connection,

    postgres: postgres::Connection
}

impl Backend {
    pub fn new() -> Backend {
        let webapi = steamwebapi::ApiClient::new(::get_apikey());
        let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let redis = redis_client.get_connection().unwrap();
        let postgres = postgres::Connection::connect(
            &::get_postgres_url() as &str,
            &postgres::SslMode::None
            ).unwrap();

        Backend {
            steam_webapi: webapi,
            redis: redis,
            postgres: postgres
        }
    }

    pub fn auth_request(&mut self, sessionid: &str) -> BackendResult<Option<SteamId>> { 
        // FIXME: delete the sessionid too.

        let steamid: Option<SteamId> = try!(self.redis.get(format!("session_steamid:{}", sessionid)));

        debug_assert_eq!(steamid.and_then(SteamId::get_universe), Some(::steamid::Universe::Public));
        debug_assert_eq!(steamid.and_then(SteamId::get_type), Some(::steamid::AccountType::Individual));

        Ok(steamid)
    }

    pub fn start_playing(&mut self, steamid: SteamId) -> BackendResult<()> {
        // okay, what are my favorite servers?
        let preferred_servers = try!(
            self.get_player_preferred_servers(steamid)
            );

        // search servers, maybe put in favorite server and return
        // now check all these servers. first, query Redis...

        // if not found, query the server directly, put in Redis,
        // use a reasonable TTL.

        // now, of those servers... are any OK?
        // if so... put them in there
        // if not... put in fallback server. then, put in queue...
        
        try!(self.put_player_in_queue(steamid, preferred_servers[0]));

        // maybe put in fallback server and queue for favorites
        //unimplemented!()
        Ok(())
    }

    pub fn put_player_in_queue(&mut self, steamid: SteamId, server: GameServerId) -> BackendResult<()> {
        try!(self.redis.sadd(format!("server_queuers:{}", server.0), steamid));
        try!(self.redis.set(format!("player_queue_info:{}", steamid.to_u64()), ""));

        Ok(())
    }

    pub fn get_player_preferred_servers(&mut self, steamid: SteamId)
        -> BackendResult<Vec<GameServerId>> {
            Ok(vec![
               GameServerId(42)
               ])
        }


    pub fn get_queue_status(&mut self, steamid: SteamId) -> BackendResult<QueueStatus> {
        let result: Option<()> = try!(self.redis.get(format!("player_queue_info:{}", steamid.to_u64())));

        Ok(match result {
            Some(()) => {
                QueueStatus::Queuing
            },
            None => {
                QueueStatus::NotQueuing
            }
        })
    }

    pub fn leave_queue(&mut self, steamid: SteamId) -> BackendResult<()> {
        unimplemented!()
    }
}
