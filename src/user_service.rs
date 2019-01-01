use bcrypt::verify;
use diesel::{
    insert_into,
    pg::PgConnection,
    prelude::*,
    r2d2::ConnectionManager,
    result::{DatabaseErrorInformation, DatabaseErrorKind, DatabaseErrorKind::*, Error}
};
use r2d2::PooledConnection;

use schema::crier_user;
use user_model::{LoginForm, LoginQuery, RegisterForm, User, UserCreation};
use user_model::UserSession;
use type_wrappers::{DBConnection, Session};
use seller_model::SellerCreation;
use mock_derive::mock;

pub struct UserService<T: UserDAO>  {
    user_dao: T
}

impl <T: UserDAO + Default> UserService<T> {
    pub fn new() -> UserService<T> {
        UserService {user_dao: T::default() }
    }

    pub fn new_with_dao(user_dao: T) {
        UserService {user_dao};
    }

    pub fn create_user(&self, conn: DBConnection, register_form: &RegisterForm, session: &mut Session) -> Result<usize, String> {

        let create_user: UserCreation = register_form.into();

        match self.user_dao.create_user(&create_user, &conn) {
                Ok(res) => {
                    session.set(UserSession {username: register_form.username.clone(), payer_id: None, seller_id: None });
                    Ok(res)
                },
                Err(e) => self.handle_insert_error(register_form, e)
            }
    }

    pub fn login(&self, conn: DBConnection, login_form: &LoginForm, session: &mut Session) -> Result<User, String> {

        let login_query:LoginQuery = login_form.into();



        let user: Result<User, String> = match self.user_dao.load_user(&login_query, &conn) {
            Ok(res) => res.clone().pop()
                .and_then(|u| {
                    match verify(login_form.password.as_str(), u.password.as_str()) {
                        Ok(true) => {
                            let user_seller: Option<i32>;
                            {
                                use schema::seller::dsl::*;
                                user_seller = self.user_dao.load_seller_id(u.id, &conn).map(|s| s.clone().pop()).ok().unwrap_or(None);
                            }
                            let user_payer: Option<i32>;
                            {
                                use schema::payer::dsl::*;
                                user_payer = self.user_dao.load_payer_id(u.id, &conn).map(|s| s.clone().pop()).ok().unwrap_or(None);
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

    pub fn get_user_from_session(&self, user_session: &UserSession, conn: &DBConnection) -> Result<User, String> {
        use schema::crier_user::dsl::*;
        crier_user.filter(username.eq(user_session.username.clone())).limit(1)
            .load::<User>(conn)
            .map(|vec| vec.clone().pop().ok_or(format!("There was no user {}", user_session.username)))
            .unwrap_or(Err(String::from("Could not load user")))
    }

    fn handle_insert_error(&self, register_form: &RegisterForm, e: Error) -> Result<usize, String> {
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

#[mock]
pub trait UserDAO {
    fn create_user(&self, user_creation: &UserCreation, conn: &DBConnection) -> QueryResult<usize>;
    fn load_user(&self, login_query: &LoginQuery, conn: &DBConnection) -> QueryResult<Vec<User>>;
    fn load_seller_id(&self, user_id: i32, conn: &DBConnection) -> QueryResult<Vec<i32>>;
    fn load_payer_id(&self, user_id: i32, conn: &DBConnection) -> QueryResult<Vec<i32>>;
}


#[cfg(not(test))]
#[derive(Default)]
#[cfg(not(test))]
pub struct UserDAOImpl {}

#[cfg(not(test))]
impl UserDAO for UserDAOImpl {
    fn create_user(&self, create_user: &UserCreation, conn: &DBConnection) -> QueryResult<usize> {
        use schema::crier_user::dsl::*;

        insert_into(crier_user)
            .values(create_user)
            .execute(conn)
    }

    fn load_user(&self, login_query: &LoginQuery, conn: &DBConnection) -> QueryResult<Vec<User>> {
        use schema::crier_user::dsl::*;

        crier_user
            .filter(username.eq(&login_query.username))
            .distinct()
            .load::<User>(conn)
    }

    fn load_seller_id(&self, user_id: i32, conn: &DBConnection) -> QueryResult<Vec<i32>> {
        use schema::seller::dsl::*;
        seller.select(id).filter(crier_user_id.eq(user_id)).distinct().load::<i32>(conn)
    }

    fn load_payer_id(&self, user_id: i32, conn: &DBConnection) -> QueryResult<Vec<i32>> {
        use schema::payer::dsl::*;
         payer.select(id).filter(crier_user_id.eq(user_id)).distinct().load::<i32>(conn)
    }
}

#[cfg(test)]
pub type UserDAOImpl = MockUserDAO;

#[cfg(test)]
impl Default for MockUserDAO {
    fn default() -> MockUserDAO {
        MockUserDAO::new()
    }
}
//
//#[cfg(test)]
//mod tests {
//
//    use super::*;
//    use std::env;
//    use dotenv::dotenv;
//
//    #[test]
//    fn test_create_user() {
//        dotenv().ok();
//        let mut user_dao = MockUserDAO::new();
//        let session = Session::new();
//        let database_url = env::var("TEST_DATABASE_URL").expect("there should be a tests db url");
//        let connection = DBConnection::establish(&database_url).unwrap();
//
//        let method_create_user = user_dao.method_create_user()
//            .called_once()
//            .return_result_of(|| Ok(1));
//
//        user_dao.set_create_user(method_create_user);
//        let mut user_service = UserService::new_with_dao(user_dao);
//        let register_form = RegisterForm {
//            username: String::from("tests"),
//            password: String::from("Password123!"),
//            password2: String::from("Password123!"),
//            email: String::from("email@testemail.com")
//        };
//        user_service.create_user(connection, register_form, session)
//    }
//}