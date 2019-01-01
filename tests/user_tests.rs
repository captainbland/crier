extern crate selenium_rs;
extern crate escargot;
use selenium_rs::webdriver::{Browser,WebDriver,Selector};
use escargot::CargoRun;
use std::thread;
use std::time;

#[test]
fn test_001_create_user() {


    println!("We're running!");

    let mut driver = WebDriver::new(Browser::Chrome);
    driver.start_session();
    driver.navigate("http://localhost:9080/register").map_err(|e| eprintln!("{}", e)).expect("Need to access web page");
    driver.query_element(Selector::CSS, "#username").unwrap().type_text("test").expect("cannot type text");
    driver.query_element(Selector::CSS, "#password").unwrap().type_text("Password123!").unwrap();
    driver.query_element(Selector::CSS, "#password2").unwrap().type_text("Password123!").unwrap();
    driver.query_element(Selector::CSS, "#email").unwrap().type_text("test@example.com").unwrap();
    driver.query_element(Selector::CSS, "#submit").unwrap().click().unwrap();

    driver.delete_session();

}

#[test]
fn test_002_login_user() {
    let mut driver = WebDriver::new(Browser::Chrome);
    driver.start_session();
    driver.navigate("http://localhost:9080/login").unwrap();
    driver.query_element(Selector::CSS, "#username").unwrap().type_text("test").unwrap();
    driver.query_element(Selector::CSS, "#password").unwrap().type_text("Password123!").unwrap();
    driver.query_element(Selector::CSS, "#submit").unwrap().click().unwrap();

}