use serde_derive::*;

use validator::*;
use validator_derive::*;

use regex::*;
use std::borrow::Cow;
use diesel::{Insertable, Queryable};
use user_model::User;
use schema::*;

#[derive(Deserialize, PartialEq, Eq, Debug, Validate)]
pub struct PayerForm {
    #[validate(length(min=2, max=128, message="Stripe source must be submitted"))]
    pub stripeSource: String,
    #[validate(length(min=2, max=128, message="Name must be between 2 and 128 characters long"))]
    pub name: String,
    #[validate(length(min=2, max=256, message="Address line must be between 2 and 256 characters long"))]
    pub addressLine1: String,
    #[validate(length(min=2, max=256, message="City must be between 2 and 256 characters long"))]
    pub city: String,
    #[validate(length(min=2, max=2, message="Country must be 2 characters long"))]
    pub country: String,
    #[validate(email)]
    pub email: String
}

#[derive(Insertable, Queryable)]
#[table_name="payer"]
pub struct PayerEntry {
    pub crier_user_id: i32,
    pub service_customer_id: String,
    pub service_payment_source: String
}