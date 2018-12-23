use serde_urlencoded::*;
use serde_derive::*;

use validator::*;
use validator_derive::*;

use regex::*;
use std::borrow::Cow;
use diesel::{Insertable, Queryable};
use regex::Regex;
use schema::listing;

//static NUMBER_REGEX: Regex = Regex::new(r"^\d+(\.\d+)?").unwrap();

#[derive(Deserialize, PartialEq, Eq, Debug, Validate)]
pub struct ListingForm {
    #[validate(length(min = 2, max = 128, message = "Title must be between 2 and 128 characters long"))]
    pub title: String,

    //#[validate(regex = "NUMBER_REGEX")]
    pub cost: String,
    pub currency: String
}

#[derive(Insertable, Queryable)]
#[table_name="listing"]
pub struct Listing {
    pub title: String,
    pub cost: i32,
    pub limited_amount: bool,
    pub currency: String
}

impl Into<Listing> for ListingForm {
    fn into(self) -> Listing {
        let cost = self.cost.replace(".", "").parse::<i32>().unwrap_or_default();
        Listing {title: self.title, cost, limited_amount: false, currency: self.currency}
    }
}