use bcrypt::verify;
use diesel::{
    insert_into,
    pg::PgConnection,
    prelude::*,
    r2d2::ConnectionManager,
    result::{DatabaseErrorInformation, DatabaseErrorKind, DatabaseErrorKind::*, Error}
};
use iron_sessionstorage::Session;
use r2d2::PooledConnection;

use schema::crier_user;
use user_model::{LoginForm, LoginQuery, RegisterForm, User, UserCreation};
use user_model::UserSession;
use seller_model::SellerCreation;

pub struct UserService;

impl UserService {
    pub fn new() -> UserService {
        UserService {}
    }

    pub fn create_user(&self, conn: PooledConnection<ConnectionManager<PgConnection>>, register_form: &RegisterForm, session: &mut Session) -> Result<usize, String> {
        use schema::crier_user::dsl::*;

        let create_user: UserCreation = register_form.into();

        match insert_into(crier_user)
            .values(create_user)
            .execute(&conn) {
                Ok(res) => {
                    session.set(UserSession {username: register_form.username.clone(), payer_id: None, seller_id: None });
                    Ok(res)
                },
                Err(e) => UserService::handle_insert_error(register_form, e)
            }
    }

    pub fn login(&self, conn: PooledConnection<ConnectionManager<PgConnection>>, login_form: &LoginForm, session: &mut Session) -> Result<User, String> {
        use schema::crier_user::dsl::*;

        let login_query:LoginQuery = login_form.into();

        let user_query = crier_user
            .filter(username.eq(&login_query.username))
            .distinct();

        let user: Result<User, String> = match user_query.load::<User>(&conn) {
            Ok(res) => res.clone().pop()
                .and_then(|u| {
                    match verify(login_form.password.as_str(), u.password.as_str()) {
                        Ok(true) => {
                            let user_seller: Option<i32>;
                            {
                                use schema::seller::dsl::*;
                                user_seller = seller.select(id).filter(crier_user_id.eq(u.id)).distinct().load::<i32>(&conn).map(|s| s.clone().pop()).ok().unwrap_or(None);
                            }
                            let user_payer: Option<i32>;
                            {
                                use schema::payer::dsl::*;
                                user_payer = payer.select(id).filter(crier_user_id.eq(u.id)).distinct().load::<i32>(&conn).map(|s| s.clone().pop()).ok().unwrap_or(None);
                            }
                            session.set(UserSession {username: login_query.username.clone(), seller_id: user_seller, payer_id: user_payer });

                            Some(Ok(u))
                        },
                        _ => Some(Err(String::from("Could not log you in")))
                    }
                })
                .unwrap_or(Err(String::from("Could not log you in"))),
            Err(e) => Err(String::from("User details not found"))
        };

        return user;

    }

    pub fn get_user_from_session(&self, user_session: &UserSession, conn: &PooledConnection<ConnectionManager<PgConnection>>) -> Result<User, String> {
        use schema::crier_user::dsl::*;
        crier_user.filter(username.eq(user_session.username.clone())).limit(1)
            .load::<User>(conn)
            .map(|vec| vec.clone().pop().ok_or(format!("There was no user {}", user_session.username)))
            .unwrap_or(Err(String::from("Could not load user")))
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


    pub fn get_integrations_for_user(&self, con: PooledConnection<ConnectionManager<PgConnection>>, user: User) {
        use schema::payer::dsl::*;

    }
}