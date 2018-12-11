use serde_urlencoded::*;
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
    stripeSource: String,
    #[validate(length(min=2, max=128, message="Name must be between 2 and 128 characters long"))]
    name: String,
    #[validate(length(min=2, max=256, message="Address line must be between 2 and 256 characters long"))]
    addressLine1: String,
    #[validate(length(min=2, max=256, message="City must be between 2 and 256 characters long"))]
    city: String,
    #[validate(length(min=2, max=2, message="Country must be 2 characters long"))]
    country: String,
    #[validate(email)]
    email: String
}