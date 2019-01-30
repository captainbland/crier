use serde_derive::*;

use validator::*;
use validator_derive::*;

use diesel::{Insertable, Queryable};
use regex::*;
use schema::*;
use std::borrow::Cow;
use user_model::User;

#[derive(Deserialize, PartialEq, Eq, Debug, Validate)]
pub struct PayForm {
    pub listing_id: i32
}