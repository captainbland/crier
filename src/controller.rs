use core::borrow::BorrowMut;
use std::env;
use std::io::Read;
use std::path::Path;
use std::string::String;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::*;
use iron::{
    *,
    modifiers::Redirect,
    prelude::*,
    status
};
use iron_sessionstorage::{
    backends::*,
    Session,
    SessionStorage,
    traits::*
};
use logger::Logger;
use maud::*;
use mount::Mount;
use params::{Params, Value};
use r2d2::Pool;
use router::Params as RouterParams;
use router::Router;
use serde_urlencoded::*;
use staticfile::Static;
use urlencoded::UrlEncodedQuery;
use validator::*;

use index_view::*;
use render_tools::*;
use navbar_info::{calculate_navbar_info, navbar_info_from_usersession};
use qr_service::*;
use qr_view::*;
use r2d2_middleware::*;
use stripe_service::*;
use stripe_service::StripeService;
use user_model::*;
use user_model::*;
use user_model::LoginForm;
use user_model::RegisterForm;
use user_model::UserSession;
use user_service::*;
use user_view::*;
use stripe_view::*;
use payer_model::PayerForm;
use type_wrappers::*;
use listing_model::*;

#[macro_use]
mod controller_macros {
    macro_rules! with_connection {
    ($req:expr, $body:expr) => {
       ($req.extensions.get::<DatabaseExtension>())
        .map(|p| p.get())
        .and_then(|res| res.ok() )
        .and_then(|con| {
            $body(con)
        });
       }
    }

    macro_rules! assert_login {
      ($session:expr, $navbar_info: expr) => {
        if !$session.get::<UserSession>().map(|r| r.is_some()).unwrap_or(false) {
            return Ok(Response::with((status::Unauthorized, render_page("You must be logged in", $navbar_info, html!{}))))
        } else {
            // if this fails it's a bug because of the above check
            $session.get::<UserSession>().unwrap()
        }
      }
    }
}

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
    Ok(Response::with((status::Ok, render_qr(svg_data.unwrap(), &navbar_info))))
}


fn post_register(req: &mut Request) -> IronResult<Response> {
    let user_form: RegisterForm = itry!(serde_urlencoded::from_reader(req.body.by_ref()));
    println!("{:?}", user_form);
    let user_service = UserService::new();
    let response: String = String::from("");
    let navbar_info = &calculate_navbar_info(req.session());

    match user_form.validate() {
        Ok(_) => {
            let res: Result<Response, IronError> = (req.extensions.get::<DatabaseExtension>())
                .map(|p| p.get())
                .and_then(|res| res.ok() )
                .and_then(|con| {
                    let insert_res = user_service.create_user(con, &user_form, req.session());
                    let result = match insert_res {
                        Ok(res) => (Ok(Response::with((status::Ok, render_post_registration_page(navbar_info))))),
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
    println!("debug: get_login");
    let session: &Session = req.session();
    let navbar_info = &calculate_navbar_info(session);

    if session.get::<UserSession>().map(|r| r.is_some()).unwrap_or(true) {
        Ok(Response::with((status::Ok, render_page("You are already logged in", navbar_info,html!{}))))
    } else {
        Ok(Response::with((status::Ok, render_login_form(navbar_info, &ValidationErrors::new()))))
    }
}

fn logout(req: &mut Request) -> IronResult<Response> {
    let session: &mut Session = req.session();

    match session.clear() {

        Ok(_) => {
            Ok(Response::with((status::Ok, render_page("You logged out!", &calculate_navbar_info(session),html!{}))))
        },
        _ => Ok(Response::with((status::InternalServerError, render_page("Could not log you out. Try again later", &calculate_navbar_info(session),html!{}))))
    }
}


fn post_login(req: &mut Request) -> IronResult<Response> {
    let user_form: LoginForm = itry!(serde_urlencoded::from_reader(req.body.by_ref()));
    let navbar_info;
    {
        let session: &Session = req.session();
        navbar_info = calculate_navbar_info(session).to_owned();
    }


    let user_service = UserService::new();
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

fn get_stripe_onboarding_url(req: &mut Request) -> IronResult<Response> {
    let navbar_info = &calculate_navbar_info(req.session());
    assert_login!(req.session(), navbar_info);

    //fixme: link this to my account...
    let stripe_client_id = match env::var("STRIPE_CLIENT_ID") {
        Ok(id) => id,
        _ => return Ok(Response::with((status::InternalServerError, "oh poo")))
    };

    let redirect_url = format!("https://connect.stripe.com/oauth/authorize?response_type=code&client_id={}&scope=read_write&redirect_uri={}{}", stripe_client_id, "http://localhost:3000/stripe/onboarding_redirect","");
    match Url::parse(redirect_url.as_str()) {
        Ok(url) => Ok(Response::with((status::Found, Redirect(url)))),
        _ => return Ok(Response::with((status::InternalServerError, "welp")))
    }
}



fn get_on_stripe_redirect(req: &mut Request) -> IronResult<Response> {
    let stripe_service = StripeService::new();
    let param_map = itry!(req.get::<Params>());
    let mut navbar_info;
    let user_session;
    {
        let session = req.session();
        navbar_info = calculate_navbar_info(session);
        user_session = assert_login!(session, &navbar_info);
    }
    let r = user_session
        .and_then(|user_session| with_connection!(req,
            |con| match param_map.find(&["code"]) {
            Some(&Value::String(ref code)) => {
                let mut session = req.session();
                stripe_service.onboard_seller(con, code, &user_session, session)

                    .map(|r| {
                        let to_return = Some(Ok((Response::with((iron::status::Ok, render_page("You've successfully been onboarded!", &navbar_info, html!{}))))));
                        navbar_info = navbar_info_from_usersession(user_session);
                        to_return
                    })
                    .map_err(|err| {
                        println!("Error onboarding: {:?}", err);
                        err
                    })
                    .unwrap_or(Some(Ok(Response::with((iron::status::InternalServerError, "There was an error...")))))
            },
            _ => Some(Ok(Response::with((iron::status::BadRequest, "Code required")))),
            }));
    return r.unwrap_or(Ok((Response::with((iron::status::BadRequest, "You're probably not logged in...")))))
}

fn get_stripe_payer_signup_form(req: &mut Request) -> IronResult<Response> {
    let navbar_info = &calculate_navbar_info(req.session());
    let user_session = assert_login!(req.session(), navbar_info);
    Ok(Response::with((iron::status::Ok, render_payer_signup_form(navbar_info, &ValidationErrors::new()))))
}

fn post_stripe_payer_signup_form(req: &mut Request) -> IronResult<Response> {
    let stripe_service = StripeService::new();
    println!("Post stripe payer signup form");
    let mut navbar_info;
    { navbar_info =  calculate_navbar_info(req.session()); }
    let user_session;
    { user_session = assert_login!(req.session(), &navbar_info).unwrap(); }
    let payer_form: PayerForm = itry!(serde_urlencoded::from_reader(req.body.by_ref()));
    match payer_form.validate() {
        Ok(_) => {
            with_connection!(req, |con| {
                let mut session = req.session();
                let res = Some(stripe_service.onboard_payer(con, payer_form, user_session.clone(), session));
                navbar_info = navbar_info_from_usersession(user_session.clone());
                res
            })
            .map(|val| Ok(Response::with((iron::status::Ok, render_page("You have created a customer!", &navbar_info, html!{})))))
            .unwrap_or(Ok(Response::with((iron::status::InternalServerError, render_page("Could not create customer", &navbar_info, html!{})))))
        },
        Err(e) => {
            println!("There was a problem: {:?}", e);
            Ok(Response::with((status::BadRequest, render_payer_signup_form(&navbar_info, &e))))
        }

    }
}

fn get_create_listing_form(req: &mut Request) -> IronResult<Response> {
    let navbar_info = &calculate_navbar_info(req.session());
    let user_session = assert_login!(req.session(), navbar_info);
    Ok(Response::with((iron::status::Ok, render_create_listing_form(navbar_info, &ValidationErrors::new()))))
}

fn post_create_listing_form(req: &mut Request) -> IronResult<Response> {
    let stripe_service = StripeService::new();
    let mut navbar_info;
    { navbar_info =  calculate_navbar_info(req.session()); }
    let user_session;
    { user_session = assert_login!(req.session(), &navbar_info).unwrap(); }
    let listing_form: ListingForm = itry!(serde_urlencoded::from_reader(req.body.by_ref()));
    match(listing_form.validate()) {
        Ok(_) => {

        }
        Err(e) => {
            Ok(Response::with((status::BadRequest, render_create_listing_form(&navbar_info, &e))))

        }
    }

}

pub fn get_router() -> Router {
    router!(
        index: get "/" => index,
        register: get "/register" => get_register,
        post_register: post "/register" => post_register,
        get_login: get "/login" => get_login,
        post_login: post "/login" => post_login,
        post_logout: post "/logout" => logout,
        qr_code: get "/qr_code" => get_qr,
        get_stripe_onboarding_url: get "/stripe/onboarding_url" => get_stripe_onboarding_url,
        stripe_redirect: get "/stripe/onboarding_redirect" => get_on_stripe_redirect,
        stripe_payee_form: get "/stripe/payer_signup" => get_stripe_payer_signup_form,
        stripe_payee_form: post "/stripe/payer_signup" => post_stripe_payer_signup_form,
        stripe_create_listing_form: get "/create_listing" => get_stripe_create_listing_form
    )
}
