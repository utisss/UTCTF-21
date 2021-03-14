#![feature(proc_macro_hygiene, decl_macro)]
#![deny(missing_debug_implementations)]

#[macro_use]
extern crate rocket;

use rocket::{
    http,
    request::{
        FromRequest,
        Outcome,
    },
    response::content,
    Request,
    State,
};

const REJECTION_PAGE: &str = include_str!("rejection.html");
const VERIFY_HEADER_NAME: &str = "X-Forwarded-SSL-Client-Verify";
const VERIFY_HEADER_SUCCESS: &str = "SUCCESS";
const DN_HEADER_NAME: &str = "X-Forwarded-SSL-Client-DN";

#[derive(Debug)]
pub struct Flag(pub String);

#[derive(Debug)]
pub struct AcceptedDn(pub String);

#[derive(Debug)]
pub struct ValidClientCert {}

impl<'a, 'r> FromRequest<'a, 'r> for ValidClientCert {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match request.guard::<State<AcceptedDn>>().and_then(|dn| {
            let dn = &(*dn).0;
            let verifies: Vec<_> =
                request.headers().get(VERIFY_HEADER_NAME).collect();
            let dns: Vec<_> = request.headers().get(DN_HEADER_NAME).collect();
            if verifies.len() == 1 &&
                verifies[0] == VERIFY_HEADER_SUCCESS &&
                dns.len() == 1 &&
                dns[0] == dn
            {
                Outcome::Success(ValidClientCert {})
            } else {
                Outcome::<_, ()>::Forward(())
            }
        }) {
            Outcome::Success(s) => Outcome::Success(s),
            _ => Outcome::Forward(()),
        }
    }
}

#[get("/", rank = 1)]
pub fn success(_cert: ValidClientCert, flag: State<Flag>) -> String {
    (*flag).0.clone()
}

#[get("/", rank = 2)]
pub fn fail<'a>() -> Unauthorized<'a> {
    Unauthorized(content::Html(REJECTION_PAGE))
}

#[derive(Responder, Debug)]
#[response(status = 401, content_type = "html")]
pub struct Unauthorized<'s>(content::Html<&'s str>);
