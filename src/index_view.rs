use maud::*;
use crate::navbar_info::NavbarInfo;
use crate::render_tools::*;

pub fn render_index(navbar_info: &NavbarInfo) -> Markup {
    render_page(
        "Hello, world!",
        navbar_info,
        html! {
            p { "Welcome to my page!" }
        },
    )
}
