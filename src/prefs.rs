use hbs::Template;
use iron::prelude::*;
use iron::status;
use Page;
use std::default::Default;

// FIXME: derived Default won't do well later
#[derive(ToJson, Default)]
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
    let new_prefs = get_prefs(&req).unwrap_or_default();

    let mut resp = Response::new();
    let data = Page {
        title: "Preferences",
        contents: PrefsPage { updated: true, prefs: new_prefs },
    }
                   .to_json();

    resp.set_mut(Template::new("display_prefs", data)).set_mut(status::Ok);
    Ok(resp)
}
