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

use rustc_serialize::json::{ToJson, Json};
use std::path::Path;
use hbs::Template;
use iron::status;
use iron::prelude::*;
use std::collections::BTreeMap;

mod prefs;


const SITEADDRESS: &'static str = "localhost:8080";

fn main() {
    env_logger::init().unwrap();

    let router = router!(
        get "/" => mainpage_handler,
        get "/prefs" => prefs::display_prefs_handler,
        post "/prefs" => prefs::update_prefs_handler
        );

    let mut mount = mount::Mount::new();
    mount.mount("/", router).mount("/css", staticfile::Static::new(Path::new("static/css")));

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
fn maybe_add_logger(_: &mut Chain) {
}

struct Page<'a, Contents> {
    title: &'a str,
    contents: Contents,
}
// #[derive] doesn't like the type parameters here
impl<'a, Contents: ToJson> Page<'a, Contents> {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("title".to_string(), self.title.to_json());
        d.insert("contents".to_string(), self.contents.to_json());
        d.to_json()
    }
}

fn mainpage_handler(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let data = Page { title: "Home", contents: () };

    resp.set_mut(Template::new("index", data.to_json())).set_mut(status::Ok);
    Ok(resp)
}
