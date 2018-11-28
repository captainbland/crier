use maud::*;
use navbar_info::NavbarInfo;

fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        html {
            meta charset="utf-8";
            link rel="stylesheet" type="text/css" href="/static/css/bootstrap.css";
            title { (page_title) }
        }
    }
}

pub fn footer() -> Markup {
    let pre_escaped = "<script src=\"/static/js/bootstrap.js\"></script>";
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
                        a class="nav-link" href="/login" { "Log in" }
                    }
                } @else {
                    li class="nav-item" {
                        form method="POST" action="/logout" { input type="submit" value="logout" {} }
                    }
                }
            }
        }
    }
}

pub fn render_page(title: &str, navbar_info: &NavbarInfo, contents: Markup) -> Markup {
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

        }
    }
}

pub fn render_index(navbar_info: &NavbarInfo) -> Markup {
    render_page("Hello, world!", navbar_info, html! {
        p { "Welcome to my page!" }
    })
}