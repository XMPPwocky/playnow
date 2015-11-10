use std;
use time;
use steamid::SteamId;

pub fn cron_thread() {
    use redis::Commands;

    let mut backend = ::create_backend_pool().get_backend(); 

    loop {
        let starttime = time::get_time().sec as f64;

        let expired_queuers: Vec<SteamId> = backend.redis.zrangebyscore(
            "players_queuing",
            std::f64::NEG_INFINITY,
            starttime).unwrap();

        for steamid in expired_queuers {
            backend.leave_queue(steamid).unwrap();
        }

        let endtime = time::get_time().sec as f64;
        let runtime = endtime - starttime;

        let waittime = f64::max(30.0 - runtime, 0.0) as u64;

        std::thread::sleep(std::time::Duration::new(waittime, 0));
    }
}
