use serde_derive::*;

use validator::*;
use validator_derive::*;

use diesel::{Insertable, Queryable};
use crate::schema::*;

#[derive(Deserialize, PartialEq, Eq, Debug, Validate)]
pub struct PayForm {
    pub listing_id: i32
}

#[derive(Deserialize, PartialEq, Eq, Debug, Validate)]
#[derive(Insertable)]
#[table_name="payment"]
pub struct PaymentEntry {
    pub payer_id: i32,
    pub seller_id: i32,
    pub listing_id: i32,
    pub cost: i32,
    pub currency: String
}

#[derive(Queryable)]
pub struct Payment {
    pub id: i32,
    pub payer_id: i32,
    pub seller_id: i32,
    pub listing_id: i32,
    pub cost: i32,
    pub currency: String
}