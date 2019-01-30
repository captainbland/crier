use serde_derive::*;

use validator::*;
use validator_derive::*;

use diesel::{Insertable, Queryable};
use regex::*;
use schema::*;
use std::borrow::Cow;
use user_model::User;

#[derive(Deserialize, PartialEq, Eq, Debug, Validate)]
pub struct PayerForm {
    #[validate(length(min = 2, max = 128, message = "Stripe source must be submitted"))]
    pub stripeSource: String,
    #[validate(length(
        min = 2,
        max = 128,
        message = "Name must be between 2 and 128 characters long"
    ))]
    pub name: String,
    #[validate(length(
        min = 2,
        max = 256,
        message = "Address line must be between 2 and 256 characters long"
    ))]
    pub addressLine1: String,
    #[validate(length(
        min = 2,
        max = 256,
        message = "City must be between 2 and 256 characters long"
    ))]
    pub city: String,
    #[validate(length(min = 2, max = 2, message = "Country must be 2 characters long"))]
    pub country: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "payer"]
pub struct PayerEntry {
    pub crier_user_id: i32,
    pub service_customer_id: String,
    pub service_payment_source: String,
}

#[derive(Queryable, Clone)]
pub struct Payer {
    pub id: i32,
    pub crier_user_id: i32,
    pub service_customer_id: Option<String>,
    pub service_payment_source: String,
}

