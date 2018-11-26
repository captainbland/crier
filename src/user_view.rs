use maud::*;
use index_view::render_page;
use validator::{ValidationErrors, ValidationError};
use std::borrow::ToOwned;
use std::borrow::Cow;

pub fn render_registration_form(errors: &ValidationErrors) -> Markup {
    render_page("Registration", html!{
        form.form-group method="POST" action="/register" {
            (render_input("Username", "username", "text", errors))
            (render_input("Password", "password", "password", errors))
            (render_input("Repeat your password", "password2", "password", errors))
            (render_input("Email", "email", "text", errors))
            input.btn.btn-primary type=("submit") value="Register";
        }
    })
}

pub fn render_login_form(errors: &ValidationErrors) -> Markup {
    render_page("Login", html! {
        form.form-group method="POST" action="/login" {
            (render_input("Username", "username", "text", errors))
            (render_input("Password", "password", "password", errors))
            input.btn.btn-primary type=("submit") value="Register";
        }
    })
}

pub fn render_input(label: &str, name: &str, input_type: &str, errors: &ValidationErrors) -> Markup {
    html! {
        div {
            label for=(name) { (label) }
            br;
            input.form-control.is-invalid[is_error_for(errors, name)] type=(input_type) name=(name) id=(name);
            (cat_errors_for_field(errors, name));
        }
    }
}

fn is_error_for(errors: &ValidationErrors, name: &str) -> bool {
    errors.to_owned().field_errors().get(name) != None
}

fn cat_errors_for_field(errors: &ValidationErrors, name: &str) -> String {
    println!("Errors: {:?}", errors);
    println!("Name: {}", name);
    let error_strings: Vec<String> = match errors.clone().field_errors().get(name) {
        Some(some_errs) => some_errs.iter()
            .map(|e| e.to_owned().message.map(|m| String::from(m)))
            .filter(Option::is_some)
            .map(Option::unwrap).collect(),

        _ => return String::from("")
    };
    println!("Errors: {:?}", error_strings);
    error_strings.join(", ")
}