use maud::*;
use navbar_info::NavbarInfo;
use render_tools::*;

pub fn render_qr(svg_data: String, navbar_info: &NavbarInfo) -> Markup {
    render_page(
        "Registration",
        navbar_info,
        html! {
            (PreEscaped(svg_data))
        },
    )
}
