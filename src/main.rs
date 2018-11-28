#![feature(plugin)]
#![feature(proc_macro_non_items)]
#![feature(extern_prelude)]

#[macro_use]
extern crate iron;
#[macro_use]
extern crate router;
extern crate plugin;
extern crate logger;
extern crate staticfile;
extern crate mount;
extern crate maud;
extern crate env_logger;
#[macro_use]
extern crate serde_derive;
extern crate serde_urlencoded;
extern crate core;
extern crate regex;
extern crate urlencoded;
extern crate qrcodegen;
extern crate lodepng;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate validator;
#[macro_use]
extern crate validator_derive;
extern crate r2d2;
extern crate bcrypt;
extern crate iron_sessionstorage;
extern crate redis;

use std::path::Path;

use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::*;
use iron_sessionstorage::traits::*;
use iron::prelude::*;
use iron::*;


use logger::Logger;
use staticfile::Static;
use iron::status;
use router::{Router};
use maud::*;
use mount::Mount;
use serde_urlencoded::*;

mod index_view;
use index_view::*;

mod user_view;
use user_view::*;

mod qr_service;
use qr_service::*;

mod qr_view;
use qr_view::*;

mod user_model;
use user_model::*;

mod user_service;
use user_service::*;

mod schema;

use core::borrow::BorrowMut;
use std::io::Read;
use std::string::String;
use urlencoded::UrlEncodedQuery;
use validator::*;

mod r2d2_middleware;
use r2d2_middleware::*;

mod navbar_info;
use navbar_info::calculate_navbar_info;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use iron_sessionstorage::Session;


fn index(req: &mut Request) -> IronResult<Response> {
    println!("plz render index plz");
    Ok(Response::with((status::Ok, render_index(&calculate_navbar_info(req.session())))))
}

fn get_register(req: &mut Request) -> IronResult<Response> {
     Ok(Response::with((status::Ok, render_registration_form(&calculate_navbar_info(req.session()), &ValidationErrors::new()))))
}

fn get_qr(req: &mut Request) -> IronResult<Response> {
    let mut qrservice = QRService{};
    let navbar_info = &calculate_navbar_info(req.session());

    let svg_data = qrservice.create_svg_data("http://www.google.com");
    Ok(Response::with((status::Ok, qr_view::render_qr(svg_data.unwrap(), &navbar_info))))
}


fn post_register(req: &mut Request) -> IronResult<Response> {
    let user_form: RegisterForm = itry!(serde_urlencoded::from_reader(req.body.by_ref()));
    println!("{:?}", user_form);
    let user_service = user_service::UserService::new();
    let mut response: String = String::from("");
    let navbar_info = &calculate_navbar_info(req.session());

    match user_form.validate() {
        Ok(_) => {
            let res: Result<Response, IronError> = (req.extensions.get::<DatabaseExtension>())
                .map(|p| p.get())
                .and_then(|res| res.ok() )
                .and_then(|con| {
                     let insert_res = user_service.create_user(con, &user_form);
                     let result = match insert_res {
                         Ok(res) => (Ok(Response::with((status::Ok, render_page("Thanks for registering!", navbar_info, html!{("Thank you, ") (user_form.username)}))))),
                         Err(e) => Ok(Response::with((status::BadRequest, render_page("There was a problem registering...", navbar_info,html!{(e.to_string())}))))
                     };
                    Some(result)
                })
                .unwrap_or( Ok(Response::with((status::InternalServerError, render_page("There was a problem registering...", navbar_info,html!{})))));
            return res;
        },

        Err(err) => Ok(Response::with((status::BadRequest, render_registration_form(navbar_info, &err))))
    }
}

fn get_login(req: &mut Request) -> IronResult<Response> {
    let mut session: &mut Session = req.session();
    let navbar_info = &calculate_navbar_info(session);

    if session.get::<UserSession>().map(|r| r.is_some()).unwrap_or(true) {
        Ok(Response::with((status::Ok, render_page("You are already logged in", navbar_info,html!{}))))
    } else {
        Ok(Response::with((status::Ok, render_login_form(navbar_info, &ValidationErrors::new()))))
    }
}

fn logout(req: &mut Request) -> IronResult<Response> {
    let session: &mut Session = req.session();
    let navbar_info = calculate_navbar_info(session).to_owned();

    match session.clear() {
        Ok(_) => Ok(Response::with((status::Ok, render_page("You logged out!", &navbar_info,html!{})))),
        _ => Ok(Response::with((status::InternalServerError, render_page("Could not log you out. Try again later", &navbar_info,html!{}))))
    }
}

fn post_login(req: &mut Request) -> IronResult<Response> {
    let user_form: LoginForm = itry!(serde_urlencoded::from_reader(req.body.by_ref()));
    println!("{:?}", user_form);
    let navbar_info;
    {
        let session: &Session = req.session();
        navbar_info = calculate_navbar_info(session).to_owned();
    }


    let user_service = user_service::UserService::new();
    let mut response: String = String::from("");
    match user_form.validate() {
        Ok(_) => {

            let res: Result<Response, IronError> = (req.extensions.get::<DatabaseExtension>())
                .map(|p| p.get())
                .and_then(|res| res.ok() )
                .and_then(|con| {

                    let mut session: &mut Session = req.session();

                    let user_res = user_service.login(con, &user_form, session);
                    let result = match user_res {
                        Ok(user) => (Ok(Response::with((status::Ok, render_page("Thanks for logging in!", &navbar_info,html!{("Thank you, ") (user_form.username)}))))),
                        Err(e) => {
                            println!("Error logging in: {:?}", e);
                            Ok(Response::with((status::BadRequest, render_page("Could not log you in, seems like your details were wrong!", &navbar_info,html!{}))))
                        }
                    };
                    return Some(result);
                }

            ).unwrap();
            return res;
        }
        _ => Ok(Response::with((status::BadRequest, render_page("Could not log you in, seems like your details were wrong!", &navbar_info,html!{}))))

    }
}

fn main() {

    let router = router!(
        index: get "/" => index,
        register: get "/register" => get_register,
        post_register: post "/register" => post_register,
        get_login: get "/login" => get_login,
        post_login: post "/login" => post_login,
        post_logout: post "/logout" => logout,
        qr_code: get "/qr_code" => get_qr
    );

    let mut mount = Mount::new();
    mount.mount("/", router)
         .mount("/static/", Static::new(Path::new("static")));


    let (logger_before, logger_after) = Logger::new(None);
    let connection_pool_middleware = r2d2_middleware::R2D2Middleware::new();
    let mut chain = Chain::new(mount);
    chain.link_before(connection_pool_middleware);
    let my_secret = b"verysecret".to_vec();
    chain.link_around(SessionStorage::new(RedisBackend::new("redis://172.17.0.4").unwrap()));
    chain.link_before(logger_before);
    chain.link_after(logger_after);


    Iron::new(chain).http("localhost:3000").unwrap();
}