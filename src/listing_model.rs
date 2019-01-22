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

#[derive(Insertable)]
#[table_name="listing"]
pub struct ListingCreation {
    pub title: String,
    pub cost: i32,
    pub limited_amount: bool,
    pub currency: String,
    pub seller_id: i32
}

#[derive(Queryable, Clone, Debug)]
pub struct Listing {
    pub id: i32,
    pub seller_id: i32,
    pub title: String,
    pub cost: i32,
    pub currency: String,
    pub amount: Option<i32>,
    pub limited_amount: Option<bool>
}

impl Into<ListingCreation> for ListingForm {
    fn into(self) -> ListingCreation {
        let cost = self.cost.replace(".", "").parse::<i32>().unwrap_or_default();
        ListingCreation {title: self.title, cost, limited_amount: false, currency: self.currency, seller_id: -1}
    }
}