use maud::*;

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

pub fn navbar() -> Markup {
    html! {
        nav class="navbar navbar-expand-sm navbar-dark bg-dark mb-2" {
            ul class="navbar-nav" {
                li class="nav-item" {
                    a class="nav-link" href="#" { "Link 1" }
                }
            }
        }
    }
}

pub fn render_page(title: &str, contents: Markup) -> Markup {
    html! {
        (header(title))
        body {
            (navbar())
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

pub fn render_index() -> Markup {
    render_page("Hello, world!", html! {
        p { "Welcome to my page!" }
    })
}