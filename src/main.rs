extern crate iron;
extern crate router;
extern crate logger;

use iron::prelude::*;
use iron::status;

const SITEADDRESS: &'static str = "localhost:8080";

fn main() {
    let mut router = router::Router::new();

    router.get("/", mainpage);

    Iron::new(router).http(SITEADDRESS).unwrap();
}

fn mainpage(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello world!")))
}

