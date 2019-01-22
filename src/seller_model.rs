use serde_derive::*;
use serde_urlencoded::*;

use validator::*;
use validator_derive::*;

use diesel::{Insertable, Queryable};
use regex::*;
use schema::*;
use std::borrow::Cow;
use user_model::User;

#[derive(Insertable)]
#[table_name = "seller"]
pub struct SellerEntry {
    pub crier_user_id: i32,
    pub access_token: String,
    pub refresh_token: String,
    pub publishable_key: String,
    pub service_id: String,
}

#[derive(Queryable, Clone)]
pub struct Seller {
    pub id: i32,
    pub crier_user_id: i32,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub publishable_key: Option<String>,
    pub service_id: String,
}
