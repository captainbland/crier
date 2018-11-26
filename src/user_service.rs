use r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;

use diesel::prelude::*;
use diesel::result::{Error, DatabaseErrorKind, DatabaseErrorKind::*, DatabaseErrorInformation};
use diesel::insert_into;
use schema::crier_user;
use bcrypt::verify;
use user_model::{RegisterForm, UserCreation, User, LoginForm, LoginQuery};

pub struct UserService;

impl UserService {
    pub fn new() -> UserService {
        UserService {}
    }

    pub fn create_user(&self, conn: PooledConnection<ConnectionManager<PgConnection>>, register_form: &RegisterForm) -> Result<usize, String> {
        use schema::crier_user::dsl::*;

        let create_user: UserCreation = register_form.into();

        match insert_into(crier_user)
            .values(create_user)
            .execute(&conn) {
                Ok(res) => Ok(res),
                Err(e) => UserService::handle_insert_error(register_form, e)
            }
    }

    pub fn login(&self, conn: PooledConnection<ConnectionManager<PgConnection>>, login_form: &LoginForm) -> Result<User, String> {
        use schema::crier_user::dsl::*;

        let login_query:LoginQuery = login_form.into();

        let user: Result<User, String> = match crier_user.filter(username.eq(&login_query.username)).limit(1).load::<User>(&conn) {
            Ok(res) => res.clone().pop()
                .and_then(|u| {
                    match verify(&login_form.password[..], &u.password[..]) {
                        Ok(true) => Some(Ok(u)),
                        _ => Some(Err(String::from("Could not log you in")))
                    }
                })
                .unwrap_or(Err(String::from("Could not log you in"))),
            Err(e) => Err(String::from("User details not found"))
        };

        return user;

    }

    fn handle_insert_error(register_form: &RegisterForm, e: Error) -> Result<usize, String> {
        println!("Error: {:?}", e);

        match e {
            Error::DatabaseError(UniqueViolation, info) => {
                let res = info.constraint_name().and_then(|s| {

                    if String::from(s).contains("username") {
                        return Some(format!("This username {} has already been used please try again.", register_form.username));
                    } else if String::from(s).contains("email") {
                        return Some(format!("This email {} has already been used please try again.", register_form.email));
                    }

                    None
                }).unwrap_or(String::from("UNKNOWN PLEASE REPORT THIS"));
                return Err(res);
            },
            _ => {
                Err(String::from("There was a problem processing your registration. Please try again later"))
            }
        }
    }
}