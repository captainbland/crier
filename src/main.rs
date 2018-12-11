#![feature(plugin)]
#![feature(proc_macro_hygiene)]
#![feature(extern_prelude)]

extern crate bcrypt;
extern crate core;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate iron;
extern crate iron_sessionstorage;
extern crate lodepng;
extern crate logger;
extern crate maud;
extern crate mount;
extern crate params;
extern crate plugin;
extern crate qrcodegen;
extern crate r2d2;
extern crate redis;
extern crate regex;
extern crate reqwest;
#[macro_use]
extern crate router;
extern crate serde_derive;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate staticfile;
extern crate stripe;
extern crate urlencoded;
extern crate validator;
#[macro_use]
extern crate validator_derive;

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

pub mod index_view;
pub mod user_view;
pub mod qr_service;
pub mod qr_view;
pub mod user_model;
pub mod user_service;
pub mod stripe_service;
pub mod controller;
pub mod seller_model;
pub mod render_tools;
pub mod stripe_view;
pub mod payer_model;

mod schema;

mod r2d2_middleware;
mod navbar_info;

fn main() {
    dotenv().ok();

    let router = controller::get_router();

    let mut mount = Mount::new();
    mount.mount("/", router)
         .mount("/static/", Static::new(Path::new("static")));


    let (logger_before, logger_after) = Logger::new(None);
    let connection_pool_middleware = r2d2_middleware::R2D2Middleware::new();
    let mut chain = Chain::new(mount);
    chain.link_before(connection_pool_middleware);
    let my_secret = b"verysecret".to_vec();
    let redis_url = env::var("REDIS_URL").unwrap();
    chain.link_around(SessionStorage::new(RedisBackend::new(redis_url.as_str()).unwrap()));
    chain.link_before(logger_before);
    chain.link_after(logger_after);


    Iron::new(chain).http("localhost:3000").unwrap();
}