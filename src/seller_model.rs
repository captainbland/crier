use serde_urlencoded::*;
use serde_derive::*;

use validator::*;
use validator_derive::*;

use regex::*;
use std::borrow::Cow;
use diesel::{Insertable, Queryable};
use user_model::User;
use schema::*;


#[derive(Insertable)]
#[table_name="seller"]
pub struct SellerCreation {
    pub crier_user_id: i32,
    pub access_token: String,
    pub refresh_token: String,
    pub publishable_key: String,
    pub service_id: String
}