use time;
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
    pub steam_webapi: steamwebapi::ApiClient,

    pub redis: redis::Connection,

    pub postgres: postgres::Connection
}

impl Backend {
    pub fn new(steam_apikey: &str,
              redis_url: &str,
              postgres_url: &str) -> Backend {
        let webapi = steamwebapi::ApiClient::new(steam_apikey.to_owned());
        let redis_client = redis::Client::open(redis_url).unwrap();
        let redis = redis_client.get_connection().unwrap();
        let postgres = postgres::Connection::connect(
            postgres_url,
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

        if let Some(steamid) = steamid {
            debug_assert_eq!(steamid.get_universe(), Some(::steamid::Universe::Public));
            debug_assert_eq!(steamid.get_type(), Some(::steamid::AccountType::Individual));
        }

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
        let exptime: f64 = time::get_time().sec as f64 + 60.0;

        try!(self.redis.zadd("players_queuing", steamid, exptime)); 

        try!(self.redis.sadd(format!("player_queuing_for:{}", steamid.to_u64()), server.0));
        try!(self.redis.set(format!("player_queue_status:{}", steamid.to_u64()), "queuing"));
        try!(self.redis.sadd(format!("server_queuers:{}", server.0), steamid));

        Ok(())
    }

    pub fn get_player_preferred_servers(&mut self, steamid: SteamId)
        -> BackendResult<Vec<GameServerId>> {
            Ok(vec![
               GameServerId(42)
               ])
        }


    pub fn get_queue_status(&mut self, steamid: SteamId) -> BackendResult<QueueStatus> {
        let status: Option<String> = try!(self.redis.get(format!("player_queue_status:{}", steamid.to_u64())));
        let status = status.as_ref().map(|s| &**s);

        Ok(match status {
            Some("queuing") => {
                let servers: Vec<GameServerId> = try!(
                    self.redis.smembers(format!("player_queuing_for:{}", steamid.to_u64()))
                    );
                QueueStatus::Queuing(servers)
            },
            _ => QueueStatus::NotQueuing
        })
    }

    pub fn leave_queue(&mut self, steamid: SteamId) -> BackendResult<()> {
        let _changes: i64 = self.redis.zrem("players_queuing", steamid).unwrap();
        let _changes: i64 = self.redis.del(format!("player_queue_status:{}", steamid.to_u64())).unwrap();
        let _changes: i64 = self.redis.del(format!("player_queuing_for:{}", steamid.to_u64())).unwrap();
        let _changes: i64 = self.redis.srem("server_queuers", steamid).unwrap();
        Ok(())
    }
}
