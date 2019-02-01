#![feature(plugin)]
#![feature(proc_macro_hygiene)]
#![warn(unused_extern_crates)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate router;


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
