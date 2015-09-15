extern crate iron;
#[macro_use]
extern crate router;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate logger;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;
extern crate staticfile;
extern crate mount;

use hbs::Template;
use iron::prelude::*;
use iron::status;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use std::path::Path;

const SITEADDRESS: &'static str = "localhost:8080";

fn main() {
    env_logger::init().unwrap();

    let router = router!(
        get "/" => mainpage,
        get "/prefs" => display_prefs,
        post "/prefs" => update_prefs
        );

    let mut mount = mount::Mount::new();
    mount
        .mount("/", router)
        .mount("/static", staticfile::Static::new(Path::new("static")));

    let mut chain = Chain::new(mount);
    chain.link_after(hbs::HandlebarsEngine::new("./templates", ".hbs"));
    maybe_add_logger(&mut chain);

    Iron::new(chain).http(SITEADDRESS).unwrap();
}

// logging in debug builds
#[cfg(debug_assertions)]
fn maybe_add_logger(chain: &mut Chain) {
    chain.link(logger::Logger::new(None));
}
// no logging in release builds
#[cfg(not(debug_assertions))]
fn maybe_add_logger(_: &mut Chain) {}

fn mainpage(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let mut data = BTreeMap::new();
    data.insert("title".to_string(), "Home".to_json());

    resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
    Ok(resp)
}

fn display_prefs(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let data = ();

    resp.set_mut(Template::new("prefs", data)).set_mut(status::Ok);
    Ok(resp)
}

fn update_prefs(_: &mut Request) -> IronResult<Response> {
    unimplemented!()
}
