use serde_urlencoded::*;
use serde_derive::*;

use validator::*;
use validator_derive::*;

use regex::*;
use std::borrow::Cow;
use diesel::{Insertable, Queryable};
use bcrypt::{DEFAULT_COST, hash, verify};

use schema::crier_user;
use iron_sessionstorage::Value;

#[cfg(feature="debug")]
const BLOWFISH_COST: u32 = 4;

#[cfg(not(feature="debug"))]
const BLOWFISH_COST: u32 = DEFAULT_COST;


#[derive(Deserialize, PartialEq, Eq, Debug, Validate)]
pub struct RegisterForm {
    #[validate(length(min=2, max=32, message="Username must be between 2 and 32 characters long"))]
    pub username: String,

    #[validate(custom="password", length(min=7, message="Password must be 7 characters or over"))]
    pub password: String,
    #[validate(must_match="password")]
    pub password2: String,
    #[validate(email)]
    pub email: String
}


fn password(password: &str) -> Result<(), ValidationError> {
    let re_vec = vec![r"[a-z]",r"[A-Z]",r"[0-9]",r"[:punct:]"];

    // count instances of the required character types
    if re_vec.iter().filter(|rexp| Regex::new(rexp).map(|re| re.is_match(password)).unwrap_or(false)).count() == re_vec.len() {
        return Ok(());
    }

    let mut error = ValidationError::new("password");
    error.message = Some(Cow::Owned(String::from("Password should contain a lower case, an upper case, a number and a punctuation character")));
    Err(error)
}

#[derive(Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String
}

#[derive(Insertable, Queryable)]
#[table_name="crier_user"]
pub struct UserCreation {
    pub username: String,
    pub password: String,
    pub email: String
}

impl<'a> Into<UserCreation> for  &'a RegisterForm {
    fn into(self) -> UserCreation {
        return UserCreation {
            username: self.username.clone(),
            password: hash(self.password.clone().as_str(), BLOWFISH_COST).unwrap_or_else(|e| {
                println!("ERROR! Could not hash password. {:?}", e);
                String::from("") // this should fail for every username; performance could be improved by not going to database but this error shouldn't actually happen unless there's a server config error
            }),
            email: self.email.clone()
        }
    }
}

#[derive(Deserialize, PartialEq, Eq, Debug, Validate)]
pub struct LoginForm {
    #[validate(length(min=2, max=32, message="Username must be between 2 and 32 characters long"))]
    pub username: String,
    #[validate(custom="password", length(min=7, message="Password must be 7 characters or over"))]
    pub password: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSession {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_id: Option<i32>
}

#[derive(Queryable)]
pub struct LoginQuery {
    pub username: String,
    pub password: String
}

impl<'a> Into<LoginQuery> for  &'a LoginForm {
    fn into(self) -> LoginQuery {
        LoginQuery {
            username: self.username.clone(),
            password: hash(self.password.clone().as_str(), BLOWFISH_COST).unwrap_or_else(|e| {
                println!("ERROR! Could not hash password. {:?}", e);
                String::from("") // this should fail for every username; performance could be improved by not going to database but this error shouldn't actually happen unless there's a server config error
            }),
        }
    }
}

impl iron_sessionstorage::Value for UserSession {
    fn get_key() -> &'static str { "logged_in_user" }
    fn into_raw(self) -> String {
        println!("INTO RAW DEBUG: {:?}", self);
        let to_return = serde_json::to_string(&self).unwrap_or(String::from(""));
        println!("INTO RAW DEBUG: {:?}", to_return);
        to_return

    }
    fn from_raw(value: String) -> Option<Self> {
        println!("FROM_RAW: {:?}", value);
        if value.is_empty() {
            None
        } else {
            serde_json::from_str(value.as_str()).ok()
        }
    }
}