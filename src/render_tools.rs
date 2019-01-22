use maud::*;
use navbar_info::NavbarInfo;
use validator::ValidationErrors;

// This file contains various utility methods for rendering web pages with forms

fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        html {
            meta charset="utf-8";
            link rel="stylesheet" type="text/css" href="/static/thirdparty/css/bootstrap.css";
            link rel="stylesheet" type="text/css" href="/static/css/styles.css";
            title { (page_title) }
        }
    }
}

pub fn footer() -> Markup {
    let pre_escaped = "<script src=\"/static/thirdparty/js/bootstrap.js\"></script><script src=\"/static/thirdparty/js/jquery.min.js\"></script><script src=\"static/js/inputs.js\"></script>";
    html! { (PreEscaped(pre_escaped)) }
}

pub fn navbar(navbar_info: &NavbarInfo) -> Markup {
    html! {
        nav class="navbar navbar-expand-sm navbar-dark bg-dark mb-2" {
            ul class="navbar-nav" {
                li class="nav-item" {
                    a class="nav-link" href="/" { "Crier" }
                }
                @if navbar_info.logged_in == false {
                    li class="nav-item" {
                        a class="nav-link" id="nav_login" href="/login" { "Log in" }
                    }

                    li class="nav-item" {
                        a class="nav-link" id="nav_register" href="/register" { "Register" }
                    }
                } @else {
                    li class="nav-item" {
                        form method="POST" action="/logout" { input type="submit" id="nav_logout" value="logout" {} }
                    }
                }

                @if !navbar_info.is_seller && navbar_info.logged_in {
                    li class="nav-item" {
                        a class="nav-link" id="nav_onboard_seller" href="/stripe/onboarding_url" {"Onboard as a seller with Stripe"}
                    }
                } @else if navbar_info.is_seller {
                    li class="nav-item" {
                        a class="nav-link" id="nav_create_listing" href="/create_listing" {"Create listing"}
                    }
                }

                @if !navbar_info.is_payer && navbar_info.logged_in {
                    li class="nav-item" {
                        a class = "nav-link" id="nav_onboard_payer" href="/stripe/payer_signup" {"Signup to pay with us"}
                    }
                }
            }
        }
    }
}

pub fn render_page(title: &str, navbar_info: &NavbarInfo, contents: Markup) -> Markup {
    render_page_with_scripts(title, navbar_info, contents, vec![])
}

pub fn render_page_with_scripts(
    title: &str,
    navbar_info: &NavbarInfo,
    contents: Markup,
    scripts: Vec<&str>,
) -> Markup {
    html! {
        (header(title))
        body {
            (navbar(navbar_info))
            div class="container" {
                div class="row" {
                    div class="col-2-sm" {}
                    div class="col-8-sm" {
                        h1 { (title) }
                        (contents)
                    }
                }
            }
            (footer())
            @for script in &scripts {
                script src=(script) {}
            }
        }
    }
}

pub fn render_input(
    label: &str,
    name: &str,
    input_type: &str,
    errors: &ValidationErrors,
) -> Markup {
    html! {
        div {
            label for=(name) { (label) }
            br;
            input.form-control.is-invalid[is_error_for(errors, name)] type=(input_type) name=(name) id=(name);
            (cat_errors_for_field(errors, name));
            br;
        }
    }
}

pub fn render_currency_input(label: &str, name: &str, errors: &ValidationErrors) -> Markup {
    html! {
        div {
            label for=(name) { (label) }
            br;
            input.form-control.currency.is-invalid[is_error_for(errors, name)] type=("number") step=("0.01") min=("1.00") max=("5000.00") name=(name) id=(name);
            (cat_errors_for_field(errors, name));
            br;
        }
    }
}

fn is_error_for(errors: &ValidationErrors, name: &str) -> bool {
    errors.to_owned().field_errors().get(name) != None
}

fn cat_errors_for_field(errors: &ValidationErrors, name: &str) -> String {
    info!("Errors: {:?}", errors);
    info!("Name: {}", name);
    let error_strings: Vec<String> = match errors.clone().field_errors().get(name) {
        Some(some_errs) => some_errs
            .iter()
            .map(|e| e.to_owned().message.map(|m| String::from(m)))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect(),

        _ => return String::from(""),
    };
    info!("Errors: {:?}", error_strings);
    error_strings.join(", ")
}
