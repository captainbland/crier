use std::env;
use std::io::Read;
use std::result::Result;

use diesel::{
    pg::PgConnection,
    r2d2::ConnectionManager,
    prelude::*
};
use r2d2::Pool;
use r2d2::PooledConnection;
use reqwest::*;
use serde_json;
use stripe::*;
use user_model::UserSession;

use seller_model::*;
use payer_model::*;
use user_service::UserService;
use diesel::insert_into;
use payer_model::PayerForm;
use std::str::FromStr;
use stripe::Error;
use user_model::User;
use type_wrappers::{DBConnection, Session};
use listing_model::*;
use user_service::UserDAOImpl;

pub struct StripeService {
    pub publishable_key: String,
    pub secret_key: String,
    pub client: reqwest::Client,
    pub user_service: UserService<UserDAOImpl>
}

impl StripeService {
    pub fn new() -> StripeService {
        let publishable_key= env::var("STRIPE_PUBLISHABLE_KEY").unwrap();
        let secret_key = env::var("STRIPE_SECRET_KEY").unwrap();
        let client = reqwest::Client::new();
        let user_service = UserService::new();
        StripeService {publishable_key, secret_key, client, user_service}
    }

    pub fn onboard_seller(&self, con: DBConnection, code: &str, user_session: &UserSession, session: &mut Session) -> std::result::Result<i32, String> {

        let url = "https://connect.stripe.com/oauth/token";

        println!("Code: {}", code);
        let params = [("code", code), ("client_secret", self.secret_key.as_str()), ("grant_type", "authorization_code")];

        let response = self.client.request(Method::POST, url).form(&params).send().and_then(|mut x| x.text()).unwrap_or(String::from("none"));
        let json: serde_json::Value = serde_json::from_str(response.as_str()).unwrap();
        let maybe_publishable_key = json["stripe_publishable_key"].as_str();
        let maybe_service_user_id = json["stripe_user_id"].as_str();
        let maybe_refresh_token = json["refresh_token"].as_str();
        let maybe_access_token = json["access_token"].as_str();

        println!("onboarding data: {:?}", response);


        match (maybe_publishable_key,
               maybe_service_user_id,
               maybe_refresh_token,
               maybe_access_token) {
            (Some(publishable_key_value), Some(service_user_id_value),
            Some(refresh_token_value), Some(access_token_value)) => {
                use schema::seller::dsl::*;


                let user;
                {
                    user = self.user_service.get_user_from_session(user_session)?;
                }

                let seller_entry = SellerCreation {
                    crier_user_id: user.id,
                    publishable_key: String::from(publishable_key_value),
                    refresh_token: String::from(refresh_token_value),
                    access_token: String::from(access_token_value),
                    service_id: String::from(service_user_id_value)
                };


                let val = insert_into(seller)
                    .values(seller_entry)
                    .returning(id)
                    .get_results(&con)
                    .map_err(|e| {
                        println!("WARN: there was an error inserting seller information {:?}", e);
                        format!("Could not insert seller information: {:?}", e)
                    }).map(|v| v.clone().pop());

                match val {
                    Ok(Some(value)) => {
                        let mut user_session_update = user_session.clone();
                        user_session_update.seller_id = Some(value);
                        session.set(user_session_update);
                        Ok(value)
                    }
                    _ => Err(String::from("Could not create seller"))
                }
            },

            _ => Err(String::from(""))
        }
    }

    pub fn onboard_payer(&self, con: DBConnection, payer_form: PayerForm, user_session: UserSession, session: &mut Session) -> std::result::Result<i32, String>  {
        use schema::payer::dsl::*;

        let mut customer_params = CustomerParams::default();
        let payment_source_params = PaymentSourceParams::Source(SourceId::from_str(payer_form.stripeSource.as_ref()).unwrap());
        customer_params.source = Some(payment_source_params);
        let customer_params_description = "A customer of some description";
        customer_params.description = Some(customer_params_description);
        let client = stripe::Client::new(self.secret_key.as_ref());
        stripe::Customer::create(&client, customer_params).and_then(|cust| {
            let user = self.user_service.get_user_from_session(&user_session).unwrap();

            let payer_entry = PayerEntry {
                crier_user_id: user.id,
                service_customer_id: cust.id.clone(),
                service_payment_source: payer_form.stripeSource
            };

            println!("Customer created: {:?}", cust.clone());
            Ok((cust, payer_entry, payer))
        }).map(|args| {

            let (_cust, payer_entry, payer) = args;
            let returned = insert_into(payer)
                .values(payer_entry)
                .returning(id)
                .get_results(&con)
                .map_err(|e| {
                    println!("WARN: there was an error inserting seller information {:?}", e);
                    format!("Could not insert seller information: {:?}", e)
                }).map(|v| v.clone().pop());

            match returned {
                Ok(Some(payer_id)) => {
                    let mut user_session_update = user_session.clone();
                    user_session_update.payer_id = Some(payer_id);
                    session.set(user_session_update);
                    Ok(payer_id)
                }
                _ => Err(String::from("Cannot get payerId"))
            }
        }).unwrap_or_else(|e|Err(format!("There was a problem creating a customer with stripe: {:?}", e)))
    }

    pub fn create_listing(&self, con: DBConnection, listing_form: ListingForm) -> Result<i32, String> {
        use schema::listing::dsl::*;
        let listing_creation: Listing = listing_form.into();

        let ret = insert_into(listing)
            .values(listing_creation)
            .returning(id)
            .get_results::<i32>(&con)
            .map_err(|e| {
                println!("WARN: there was a database error creating listing from form: {:?}", e);
                format!("There was a problem creating listing information")
            }).map(|v| v.clone().pop().unwrap());
            //.unwrap_or_else(|e| Err(format!("There was a problem creating a listing: {:?}", e)));

        ret
    }
}