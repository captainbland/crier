use serde_urlencoded::*;
use serde_derive::*;

use validator::*;
use validator_derive::*;

use regex::*;
use std::borrow::Cow;
use diesel::{Insertable, Queryable};

#[derive(Deserialize, PartialEq, Eq, Debug, Validate)]
pub struct ListingForm {
    #[validate(length(min = 2, max = 128, message = "Title must be between 2 and 128 characters long"))]
    pub title: String,

    #[validate(regex = r"\d+(\.\d+)?")]
    pub cost: String,
    pub amount: i32,
    pub limitedAmount: bool
}
