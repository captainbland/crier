#![feature(plugin)]
#![feature(proc_macro_hygiene)]
#![feature(extern_prelude)]
extern crate bcrypt;
extern crate core;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate log;
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
extern crate r2d2_postgres;
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
#[macro_use]
extern crate mock_derive;

use crate::application::run;

pub mod controller;
pub mod index_view;
pub mod listing_model;
pub mod payer_model;
pub mod qr_service;
pub mod qr_view;
pub mod render_tools;
pub mod seller_model;
pub mod stripe_service;
pub mod stripe_view;
pub mod type_wrappers;
pub mod user_model;
pub mod user_service;
pub mod user_view;

pub mod schema;

pub mod navbar_info;
pub mod r2d2_middleware;

pub mod application;
pub mod payment_model;

pub fn main() {
    run();
}
