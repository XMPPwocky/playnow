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
extern crate steamid;
extern crate plugin;
extern crate playnow_core;

use rustc_serialize::json::{ToJson, Json};
use std::path::Path;
use hbs::Template;
use iron::status;
use iron::prelude::*;
use std::collections::BTreeMap;
use playnow_core::backend;

mod prefs;
mod login;

const SITEADDRESS: &'static str = "localhost:8080";

fn get_cookie_signing_key() -> Vec<u8> {
    b"yargh i'm a terrible signing key don't use me".to_vec()
}

fn main() {
    env_logger::init().unwrap();

    let router = router!(
        get "/" => mainpage,
        get "/prefs" => prefs::display_prefs,
        post "/prefs" => prefs::update_prefs,
        get "/login" => login::display_login,
        post "/login" => login::process_login
        );

    let mut mount = mount::Mount::new();
    mount.mount("/", router).mount("/css", staticfile::Static::new(Path::new("static/css")));

    let mut chain = Chain::new(mount);
    chain.link_before(BackendPoolMiddleware { pool: backend::create_backend_pool() });
    chain.link(oven::new(get_cookie_signing_key()));
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

struct BackendPoolMiddleware {
    pool: backend::BackendPool
}
impl iron::BeforeMiddleware for BackendPoolMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<RequestBackendPool>(self.pool.clone());
        Ok(())
    }
}
struct RequestBackendPool;
impl iron::typemap::Key for RequestBackendPool {
    type Value = backend::BackendPool;
}

struct RequestBackend;
impl iron::typemap::Key for RequestBackend {
    type Value = backend::Backend;
}

impl<'a, 'b> plugin::Plugin<Request<'a, 'b>> for RequestBackend {
    type Error = ();
    fn eval(req: &mut Request) -> Result<backend::Backend, ()> {
        match req.extensions.get::<RequestBackendPool>() {
            Some(pool) => Ok(pool.get_backend()),
            None => Err(())
        }
    }
}
            

fn mainpage(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let data = Page { title: "Home", contents: () };

    resp.set_mut(Template::new("index", data.to_json())).set_mut(status::Ok);
    Ok(resp)
}
