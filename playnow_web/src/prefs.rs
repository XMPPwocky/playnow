use hbs::Template;
use iron::prelude::*;
use iron::status;
use cookie::Cookie;
use oven::prelude::*;
use Page;
use urlencoded::UrlEncodedBody;
use rustc_serialize::json::{self, ToJson};

// FIXME: derived Default won't do well later
#[derive(ToJson, RustcDecodable, Default, Clone, Debug)]
pub struct Prefs {
    pub foo: bool,
}

pub fn get_prefs(req: &mut Request) -> Option<Prefs> {
    req.get_cookie("playnow_prefs")
       .and_then(|cookie| json::decode::<Prefs>(&cookie.value).ok())
}

#[derive(ToJson)]
pub struct PrefsPage {
    prefs: Prefs,
    updated: bool,
}


pub fn display_prefs(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let data = Page {
        title: "Preferences",
        contents: PrefsPage {
            updated: false,
            prefs: get_prefs(req).unwrap_or_default(),
        },
    }
                   .to_json();

    resp.set_mut(Template::new("display_prefs", data)).set_mut(status::Ok);
    Ok(resp)
}

pub fn update_prefs(req: &mut Request) -> IronResult<Response> {
    // FIXME: Need CSRF token here!!!
    let new_prefs = match req.get_ref::<UrlEncodedBody>() {
        Ok(ref hashmap) => {
            Some(Prefs {
                foo: hashmap.get("foo")
                            .and_then(|x| x.get(0))
                            .map(|x| {
                                match x as &str {
                                    "on" => true,
                                    _ => false,
                                }
                            })
                            .unwrap_or(false),
            })
        }
        Err(_) => {
            None
        }
    };
    let updated = new_prefs.is_some();
    let new_prefs = new_prefs.or(get_prefs(req)).unwrap_or_default();

    let mut resp = Response::new();
    let data = Page {
        title: "Preferences",
        contents: PrefsPage { updated: updated, prefs: new_prefs.clone() },
    }
                   .to_json();

    resp.set_mut(Template::new("display_prefs", data)).set_mut(status::Ok);

    resp.set_cookie(
        Cookie::new(
                    "playnow_prefs".to_string(),
                    new_prefs.to_json().to_string()
        )
    );

    Ok(resp)
}
