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
use url::Url;
use user_model::UserSession;

use seller_model::*;
use user_service::UserService;
use diesel::insert_into;


pub struct StripeService {
    pub publishable_key: String,
    pub secret_key: String,
    pub client: reqwest::Client,
    pub user_service: UserService
}

impl StripeService {
    pub fn new() -> StripeService {
        let publishable_key= env::var("STRIPE_PUBLISHABLE_KEY").unwrap();
        let secret_key = env::var("STRIPE_SECRET_KEY").unwrap();
        let client = reqwest::Client::new();
        let user_service = UserService::new();
        StripeService {publishable_key, secret_key, client, user_service}
    }

    pub fn onboard_seller(&self, con: PooledConnection<ConnectionManager<PgConnection>>, code: &str, user_session: &UserSession) -> std::result::Result<usize, String> {

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
                    user = self.user_service.get_user_from_session(user_session, &con)?;
                }

                let seller_entry = SellerCreation {
                    crier_user_id: user.id,
                    publishable_key: String::from(publishable_key_value),
                    refresh_token: String::from(refresh_token_value),
                    access_token: String::from(access_token_value),
                    service_id: String::from(service_user_id_value)
                };


                insert_into(seller)
                    .values(seller_entry)
                    .execute(&con)
                    .map_err(|e| {
                        println!("WARN: there was an error inserting seller information {:?}", e);
                        format!("Could not insert seller information: {:?}", e)
                    })

            },

            _ => Err(String::from(""))
        }
    }
}