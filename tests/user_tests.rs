extern crate escargot;
extern crate selenium_rs;
use escargot::CargoRun;
use selenium_rs::webdriver::{Browser, Selector, WebDriver};
use std::thread;
use std::time;

#[test]
fn test_001_create_user() {
    println!("We're running!");

    let mut driver = WebDriver::new(Browser::Chrome);
    driver.start_session();
    driver
        .navigate("http://localhost:9080/register")
        .map_err(|e| println!("{}", e))
        .expect("Need to access web page");
    driver
        .query_element(Selector::CSS, "#username")
        .unwrap()
        .type_text("test")
        .expect("cannot type text");
    driver
        .query_element(Selector::CSS, "#password")
        .unwrap()
        .type_text("Password123!")
        .unwrap();
    driver
        .query_element(Selector::CSS, "#password2")
        .unwrap()
        .type_text("Password123!")
        .unwrap();
    driver
        .query_element(Selector::CSS, "#email")
        .unwrap()
        .type_text("test@example.com")
        .unwrap();
    driver
        .query_element(Selector::CSS, "#submit")
        .unwrap()
        .click()
        .unwrap();
    driver
        .query_element(Selector::CSS, "#success")
        .expect("Should succeed");
    driver.delete_session();
}

#[test]
fn test_002_login_user() {
    let mut driver = WebDriver::new(Browser::Chrome);
    driver.start_session();
    login_user(&driver);
    driver
        .query_element(Selector::CSS, "#success")
        .expect("Should succeed");
}

fn login_user(driver: &WebDriver) {
    driver.navigate("http://localhost:9080/login").unwrap();
    driver
        .query_element(Selector::CSS, "#username")
        .unwrap()
        .type_text("test")
        .unwrap();
    driver
        .query_element(Selector::CSS, "#password")
        .unwrap()
        .type_text("Password123!")
        .unwrap();
    driver
        .query_element(Selector::CSS, "#submit")
        .unwrap()
        .click()
        .unwrap();
}

fn click(driver: &WebDriver, selector: &str) {
    driver
        .query_element(Selector::CSS, selector)
        .unwrap()
        .click()
        .unwrap();
}

fn type_text(driver: &WebDriver, selector: &str, text: &str) {
    driver
        .query_element(Selector::CSS, selector)
        .unwrap()
        .type_text(text)
        .unwrap();
}

#[test]
fn test_003_onboard_seller() {
    let mut driver = WebDriver::new(Browser::Chrome);
    driver.start_session();
    login_user(&driver);
    driver
        .navigate("http://localhost:9080/stripe/onboarding_url")
        .unwrap();

    click(&driver, "#skip-account-app");
    driver.navigate("http://localhost:9080/create_listing");

    type_text(&driver, "#title", "product");
    type_text(&driver, "#cost", "10.00");
    type_text(&driver, "#quantity", "1");
    type_text(&driver, "#currency", "GBP");
    driver
        .query_element(Selector::CSS, "#submit")
        .unwrap()
        .click()
        .unwrap();
}

#[test]
fn test_004_onboard_payer() {
    let driver = WebDriver::new(Browser::Chrome);

    driver.navigate("http://localhost:9080/stripe/onboard_payer");
    type_text(&driver, "#name", "someone")
}
