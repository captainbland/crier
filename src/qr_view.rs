use maud::*;
use index_view::*;

pub fn render_qr(svg_data: String) -> Markup {
    render_page("Registration", html!{
        (PreEscaped(svg_data))
    })
}