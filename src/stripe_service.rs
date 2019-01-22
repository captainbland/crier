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
    pub user_service: UserService<UserDAOImpl>,
    pub stripe_dao: StripeDAOImpl
}

impl StripeService {
    pub fn new() -> StripeService {
        let publishable_key= env::var("STRIPE_PUBLISHABLE_KEY").unwrap();
        let secret_key = env::var("STRIPE_SECRET_KEY").unwrap();
        let client = reqwest::Client::new();
        let user_service = UserService::new();
        let stripe_dao = StripeDAOImpl::new();
        StripeService {publishable_key, secret_key, client, user_service, stripe_dao}
    }

    pub fn onboard_seller(&self, con: DBConnection, code: &str, user_session: &UserSession, session: &mut Session) -> std::result::Result<i32, String> {

        let url = "https://connect.stripe.com/oauth/token";

        info!("Code: {}", code);
        let params = [("code", code), ("client_secret", self.secret_key.as_str()), ("grant_type", "authorization_code")];

        let response = self.client.request(Method::POST, url).form(&params).send().and_then(|mut x| x.text()).unwrap_or(String::from("none"));
        let json: serde_json::Value = serde_json::from_str(response.as_str()).unwrap();
        let maybe_publishable_key = json["stripe_publishable_key"].as_str();
        let maybe_service_user_id = json["stripe_user_id"].as_str();
        let maybe_refresh_token = json["refresh_token"].as_str();
        let maybe_access_token = json["access_token"].as_str();

        info!("onboarding data: {:?}", response);


        match (maybe_publishable_key,
               maybe_service_user_id,
               maybe_refresh_token,
               maybe_access_token) {
            (Some(publishable_key_value), Some(service_user_id_value),
            Some(refresh_token_value), Some(access_token_value)) => {

                let user;
                {
                    user = self.user_service.get_user_from_session(user_session, &con)?;
                }

                let seller_entry = SellerEntry {
                    crier_user_id: user.id,
                    publishable_key: String::from(publishable_key_value),
                    refresh_token: String::from(refresh_token_value),
                    access_token: String::from(access_token_value),
                    service_id: String::from(service_user_id_value)
                };


                let val = self.stripe_dao.create_seller(seller_entry, &con);


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
            let user = self.user_service.get_user_from_session(&user_session, &con).unwrap();

            let payer_entry = PayerEntry {
                crier_user_id: user.id,
                service_customer_id: cust.id.clone(),
                service_payment_source: payer_form.stripeSource
            };

            info!("Customer created: {:?}", cust.clone());
            Ok((cust, payer_entry, payer))
        }).map(|args| {

            let (_cust, payer_entry, payer) = args;
            let returned = self.stripe_dao.create_payer(payer_entry, &con);

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

    pub fn create_listing(&self, con: DBConnection, listing_form: ListingForm, sellerid: i32) -> Result<i32, String> {
        let mut listing_creation: ListingCreation = listing_form.into();
        listing_creation.seller_id = sellerid;
        self.stripe_dao.create_listing(listing_creation, &con)
    }

    pub fn get_listing(&self, con: DBConnection, listing_id: i32) -> Result<Listing, String>  {
        let res = self.stripe_dao.get_listing(listing_id, &con);
        info!("listing: {:?}", res);
        res
    }

    pub fn pay_listing(&self, con: DBConnection, payer_user_id: i32, listing_id: i32) -> Result<String, String> {
        unimplemented!()
    }
}

pub trait StripeDAO {
    fn create_seller(&self, seller_entry: SellerEntry, conn: &DBConnection) -> Result<Option<i32>, String>;
    fn create_payer(&self, payer: PayerEntry, conn: &DBConnection) -> Result<Option<i32>, String>;
    fn create_listing(&self, listing: ListingCreation, conn: &DBConnection) -> Result<i32, String>;
    fn get_listing(&self, listing_id: i32, conn: &DBConnection) -> Result<Listing, String>;

    fn get_payer_by_user_id(&self, payer_user_id: i32, conn: &DBConnection) -> Result<Payer, String>;
    fn get_seller(&self, seller_id: i32, conn: &DBConnection) -> Result<Seller, String>;
}

pub struct StripeDAOImpl {}


impl StripeDAOImpl {
    pub fn new() -> StripeDAOImpl { StripeDAOImpl {} }
}

impl StripeDAO for StripeDAOImpl {
    fn create_seller(&self, seller_entry: SellerEntry, conn: &DBConnection) -> Result<Option<i32>, String> {
        use schema::seller::dsl::*;

        insert_into(seller)
            .values(seller_entry)
            .returning(id)
            .get_results(conn)
            .map_err(|e| {
                info!("WARN: there was an error inserting seller information {:?}", e);
                format!("Could not insert seller information: {:?}", e)
            }).map(|v| v.clone().pop())
    }

    fn create_payer(&self, payer_entry: PayerEntry, conn: &DBConnection) -> Result<Option<i32>, String> {
        use schema::payer::dsl::*;

        insert_into(payer)
            .values(payer_entry)
            .returning(id)
            .get_results(conn)
            .map_err(|e| {
                info!("WARN: there was an error inserting seller information {:?}", e);
                format!("Could not insert seller information: {:?}", e)
            }).map(|v| v.clone().pop())
    }

    fn create_listing(&self, listing_creation: ListingCreation, conn: &DBConnection) -> Result<i32, String> {
        use schema::listing::dsl::*;

        insert_into(listing)
            .values(listing_creation)
            .returning(id)
            .get_results::<i32>(conn).map_err(|e| {
            info!("WARN: there was a database error creating listing from form: {:?}", e);
            format!("There was a problem creating listing information")
        }).map(|v| v.clone().pop().unwrap())
    }

    fn get_listing(&self, listing_id: i32, conn: &DBConnection) -> Result<Listing, String> {
        use schema::listing::dsl::*;
        match listing.filter(id.eq(listing_id))
            .load::<Listing>(conn)
            .map(|v| v.clone().pop()) {
            Ok(Some(res)) => Ok(res),
            _ => Err(String::from("Could not get listing"))
        }
    }

    fn get_payer_by_user_id(&self, payer_user_id: i32, conn: &DBConnection) -> Result<Payer, String> {
        use schema::payer::dsl::*;
        match payer.filter(crier_user_id.eq(payer_user_id))
            .load::<Payer>(conn)
            .map(|v| v.clone().pop()) {
            Ok(Some(res)) => Ok(res),
            _ => Err(String::from("Could not get payer"))
        }
    }

    fn get_seller(&self, seller_id: i32, conn: &DBConnection) -> Result<Seller, String> {
        use schema::seller::dsl::*;
        match seller.filter(id.eq(seller_id))
            .load::<Seller>(conn)
            .map(|v| v.clone().pop()) {
            Ok(Some(res)) => Ok(res),
            _ => Err(String::from("Could not get seller"))
        }
    }
}