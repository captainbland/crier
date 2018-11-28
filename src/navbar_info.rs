use iron_sessionstorage::*;
use user_model::UserSession;

#[derive(Copy)]
pub struct NavbarInfo {
    pub logged_in: bool
}

impl Clone for NavbarInfo {
    fn clone(&self) -> NavbarInfo { NavbarInfo { logged_in: self.logged_in } }
}

pub fn calculate_navbar_info(session: &Session) -> NavbarInfo {
    let logged_in = session.get::<UserSession>().map(|r| r.is_some()).unwrap_or(true);
    NavbarInfo {logged_in}
}