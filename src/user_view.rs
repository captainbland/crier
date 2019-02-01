use maud::*;
use crate::navbar_info::NavbarInfo;
use crate::render_tools::*;
use validator::{ValidationErrors};

pub fn render_registration_form(navbar_info: &NavbarInfo, errors: &ValidationErrors) -> Markup {
    render_page(
        "Registration",
        navbar_info,
        html! {
            form.form-group method="POST" action="/register" {
                (render_input("Username", "username", "text", errors))
                (render_input("Password", "password", "password", errors))
                (render_input("Repeat your password", "password2", "password", errors))
                (render_input("Email", "email", "text", errors))
                input.btn.btn-primary type=("submit") id="submit" value="Register";
            }
        },
    )
}

pub fn render_login_form(navbar_info: &NavbarInfo, errors: &ValidationErrors) -> Markup {
    render_page(
        "Login",
        navbar_info,
        html! {
            form.form-group method="POST" action="/login" {
                (render_input("Username", "username", "text", errors))
                (render_input("Password", "password", "password", errors))
                input.btn.btn-primary type=("submit") id="submit" value="Log in";
            }
        },
    )
}

pub fn render_post_registration_page(navbar_info: &NavbarInfo) -> Markup {
    render_page(
        "You have registered successfully",
        navbar_info,
        html! {
            p{("Now would you like to: ")}
            p {a href="/stripe/payer_signup" {("Make payments")} }
            p {a href="/stripe/onboarding_url" {("Take payments with Stripe")} }
        },
    )
}
