use hbs::Template;
use iron::prelude::*;
use iron::status;
use cookie::Cookie;
use oven::prelude::*;
use Page;
use urlencoded::UrlEncodedBody;
use rustc_serialize::json::{self, ToJson};

pub fn display_login_handler(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let data = Page { contents: () }.to_json();

    resp.set_mut(Template::new("login", data)).set_mut(status::Ok);
    Ok(resp)
}

pub fn process_login_handler(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    // FIXME: Need CSRF token here!!!
    let new_steamid = req.get_ref::<UrlEncodedBody>()
                         .ok()
                         .and_then(|hashmap| hashmap.get("steamid"))
                         .and_then(|x| x.get(0))
                         .cloned()
                         .unwrap_or(String::new());;

    resp.set_cookie(
                Cookie::new("playnow_steamid".to_string(), new_steamid));

    let mut resp = Response::new();
    let data = Page { contents: () }.to_json();

    resp.set_mut(Template::new("login", data)).set_mut(status::Ok);

    Ok(resp)
}
