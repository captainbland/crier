use std::env;
use std::path::Path;

use dotenv::*;
use iron::prelude::*;
use iron_sessionstorage::backends::*;
use iron_sessionstorage::SessionStorage;
use logger::Logger;
use mount::Mount;
use staticfile::Static;

use crate::controller;
use crate::r2d2_middleware;

pub fn run() {
    dotenv().ok();
    env_logger::init();
    let router = controller::get_router();

    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/static/", Static::new(Path::new("static")));

    let (logger_before, logger_after) = Logger::new(None);
    let connection_pool_middleware = r2d2_middleware::R2D2Middleware::new();
    let mut chain = Chain::new(mount);
    chain.link_before(connection_pool_middleware);
    let _my_secret = b"verysecret".to_vec();
    let redis_url = env::var("REDIS_URL").unwrap();
    chain.link_around(SessionStorage::new(
        RedisBackend::new(redis_url.as_str()).unwrap(),
    ));
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain).http("0.0.0.0:9080").unwrap();
}
