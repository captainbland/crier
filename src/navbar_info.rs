use type_wrappers::*;
use user_model::UserSession;
use iron::IronResult;

#[derive(Copy, Clone)]
pub struct NavbarInfo {
    pub logged_in: bool,
    pub is_seller: bool,
    pub is_payer: bool
}

pub fn calculate_navbar_info(session: &Session) -> NavbarInfo {
    let maybe_user_session: IronResult<Option<UserSession>> = session.get::<UserSession>();
    let mut to_return: NavbarInfo = NavbarInfo { logged_in: false, is_seller: false, is_payer: false };
    match maybe_user_session {
        Ok(Some(user_session)) => {
            to_return = navbar_info_from_usersession(user_session)
        }
        Err(e) => println!("Warn: There was an error getting session data {:?}", e),
        _ =>  println!("Debug: Could not get session data"),
    }
    to_return

}

pub fn navbar_info_from_usersession(user_session: UserSession) -> NavbarInfo {
    let logged_in = true;
    let is_seller = user_session.seller_id.is_some();
    let is_payer = user_session.payer_id.is_some();
    NavbarInfo {logged_in, is_seller, is_payer}
}