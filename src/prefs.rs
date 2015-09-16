use hbs::Template;
use iron::prelude::*;
use iron::status;
use iron::headers;
use cookie::Cookie;
use Page;
use std::default::Default;
use urlencoded::UrlEncodedBody;
use rustc_serialize::json::ToJson;

// FIXME: derived Default won't do well later
#[derive(ToJson, Default, Clone)]
pub struct Prefs {
    pub foo: bool,
}

pub fn get_prefs(_req: &Request) -> Option<Prefs> {
    // FIXME: pull it out of the request
    Some(Default::default())
}

#[derive(ToJson)]
pub struct PrefsPage {
    prefs: Prefs,
    updated: bool,
}


pub fn display_prefs_handler(req: &mut Request) -> IronResult<Response> {
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

pub fn update_prefs_handler(req: &mut Request) -> IronResult<Response> {
    // FIXME: Need CSRF token here!!!
    // FIXME: change this to parse out of body & set cookies
    let new_prefs = match req.get_ref::<UrlEncodedBody>() {
        Ok(ref hashmap) => {
            Some(Prefs {
                foo: hashmap.get("foo")
                            .and_then(|x| x.get(0))
                            .and_then(|x| ::std::str::FromStr::from_str(x).ok())
                            .unwrap_or(false),
            })
        }
        Err(e) => {
            None
        }
    };
    let updated = new_prefs.is_some();
    let new_prefs = new_prefs.or(get_prefs(&req)).unwrap_or_default();

    let mut resp = Response::new();
    let data = Page {
        title: "Preferences",
        contents: PrefsPage { updated: updated, prefs: new_prefs.clone() },
    }
                   .to_json();

    resp.set_mut(Template::new("display_prefs", data)).set_mut(status::Ok);
    resp.headers.set(headers::SetCookie(vec![Cookie::new(
        "playnow_prefs".to_string(),
        new_prefs.to_json().to_string(),
        )]));


    Ok(resp)
}
