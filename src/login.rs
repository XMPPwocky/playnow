use hbs::Template;
use iron::prelude::*;
use iron::status;
use cookie::Cookie;
use oven::prelude::*;
use Page;
use url;
use urlencoded::UrlEncodedBody;
use rustc_serialize::json::{self, ToJson};
use std::collections::BTreeMap;

#[derive(Debug, ToJson)]
struct LoginPage {
    loginurl: String,
}

pub fn display_login_handler(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    // build login URL
    let returnto = req.url.clone();
    let mut realm = req.url.clone();
    realm.path = vec![String::new()];
    // FIXME: this is the worst
    match realm.host {
        url::Host::Domain(ref s) => if s == "127.0.0.1" || s == "playnow.xmppwocky.net" {
            ()
        } else {
            panic!()
        },
        _ => panic!()
    };
    realm.fragment = None;
    realm.query = None;

    let mut params = BTreeMap::new();
    params.insert("openid.ns", "http://specs.openid.net/auth/2.0".to_string());
    params.insert("openid.mode", "checkid_setup".to_string());
    params.insert("openid.return_to", returnto.into_generic_url().serialize());
    params.insert("openid.realm", realm.into_generic_url().serialize());
    params.insert("openid.identity", "http://specs.openid.net/auth/2.0/identifier_select".to_string());
    params.insert("openid.claimed_id", "http://specs.openid.net/auth/2.0/identifier_select".to_string());

    let loginparams = url::form_urlencoded::serialize(params);
    let data = Page { contents: LoginPage { loginurl: "https://steamcommunity.com/openid/login/?".to_string() + &loginparams } }.to_json();

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

    resp.set_cookie(Cookie::new("playnow_steamid".to_string(), new_steamid));

    let mut resp = Response::new();
    let data = Page { contents: () }.to_json();

    resp.set_mut(Template::new("login", data)).set_mut(status::Ok);

    Ok(resp)
}
