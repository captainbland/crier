
use core::borrow::BorrowMut;
use std::env;
use std::io::Read;
use std::path::Path;
use std::string::String;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::*;
use iron::*;
use iron::modifiers::Redirect;
use iron::prelude::*;
use iron::status;
use iron_sessionstorage::backends::*;
use iron_sessionstorage::Session;
use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::traits::*;
use logger::Logger;
use maud::*;
use mount::Mount;
use params::{Params, Value};
use router::Router;
use router::Params as RouterParams;
use serde_urlencoded::*;
use staticfile::Static;
use urlencoded::UrlEncodedQuery;
use validator::*;

use controller::*;
use r2d2_middleware;
use controller;

pub fn run() {
    dotenv().ok();

    let router = controller::get_router();

    let mut mount = Mount::new();
    mount.mount("/", router)
        .mount("/static/", Static::new(Path::new("static")));


    let (logger_before, logger_after) = Logger::new(None);
    let connection_pool_middleware = r2d2_middleware::R2D2Middleware::new();
    let mut chain = Chain::new(mount);
    chain.link_before(connection_pool_middleware);
    let _my_secret = b"verysecret".to_vec();
    let redis_url = env::var("REDIS_URL").unwrap();
    chain.link_around(SessionStorage::new(RedisBackend::new(redis_url.as_str()).unwrap()));
    chain.link_before(logger_before);
    chain.link_after(logger_after);


    Iron::new(chain).http("0.0.0.0:9080").unwrap();
}