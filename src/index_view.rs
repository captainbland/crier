use maud::*;
use navbar_info::NavbarInfo;
use render_tools::*;


pub fn render_index(navbar_info: &NavbarInfo) -> Markup {
    render_page("Hello, world!", navbar_info, html! {
        p { "Welcome to my page!" }
    })
}