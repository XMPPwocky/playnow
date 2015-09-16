#![feature(custom_derive, plugin)]
#![plugin(tojson_macros)]

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
extern crate cookie;
extern crate urlencoded;
extern crate oven;

use rustc_serialize::json::{ToJson, Json};
use std::path::Path;
use hbs::Template;
use iron::status;
use iron::prelude::*;
use std::collections::BTreeMap;

mod prefs;
mod login;

const SITEADDRESS: &'static str = "localhost:8080";

fn get_cookie_signing_key() -> Vec<u8> {
    b"yargh i'm a terrible signing key don't use me".to_vec()
}

fn main() {
    env_logger::init().unwrap();

    let router = router!(
        get "/" => mainpage_handler,
        get "/prefs" => prefs::display_prefs_handler,
        post "/prefs" => prefs::update_prefs_handler,
        get "/login" => login::display_login_handler,
        post "/login" => login::process_login_handler
        );

    let mut mount = mount::Mount::new();
    mount.mount("/", router).mount("/css", staticfile::Static::new(Path::new("static/css")));

    let mut chain = Chain::new(mount);
    chain.link(oven::create(get_cookie_signing_key()));
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
fn maybe_add_logger(_: &mut Chain) {
}

struct Page<Contents> {
    contents: Contents,
}
// #[derive] doesn't like the type parameters here
impl<Contents: ToJson> Page<Contents> {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("contents".to_string(), self.contents.to_json());
        d.to_json()
    }
}

fn mainpage_handler(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let data = Page { contents: () };

    resp.set_mut(Template::new("index", data.to_json())).set_mut(status::Ok);
    Ok(resp)
}
