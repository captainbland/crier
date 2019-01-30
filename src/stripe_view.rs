use listing_model::Listing;
use maud::*;
use navbar_info::NavbarInfo;
use render_tools::*;
use validator::ValidationErrors;

pub fn render_payer_signup_form(navbar_info: &NavbarInfo, errors: &ValidationErrors) -> Markup {
    render_page_with_scripts(
        "Register to pay with us",
        navbar_info,
        html! {
            form.form-group id="payment-form" method="POST" action="/stripe/payer_signup" {
                (render_input("Real name", "name", "text", errors))
                (render_input("Address line 1", "addressLine1", "text", errors))
                (render_input("City", "city", "text", errors))
                (render_input("Post code", "postCode", "text", errors))

                (render_input("Country", "country", "text", errors))
                (render_input("Email", "email", "text", errors))
                span {("Card")}
                br{}
                div id="card-element" {}
                input.btn.btn-primary type=("submit") value="Pay with us";
            }
        },
        vec![
            "https://js.stripe.com/v3/",
            "/static/js/create_stripe_source.js",
        ],
    )
}

pub fn render_create_listing_form(navbar_info: &NavbarInfo, errors: &ValidationErrors) -> Markup {
    render_page(
        "Create listing",
        navbar_info,
        html! {
            form.form-group method="POST" action="/create_listing" {
                (render_input("Title", "title", "text", errors))
                (render_currency_input("Cost", "cost", errors))
                (render_input("Quantity", "quantity", "number", errors))
                (render_input("Currency", "currency", "text", errors))
                input.btn.btn-primary type=("submit") id="submit" value="Register";
            }
        },
    )
}

pub fn render_listing(navbar_info: &NavbarInfo, listing: Listing, qr_data: String) -> Markup {
    render_page(
        "Listing",
        navbar_info,
        html! {
            h2 {(listing.title)}
            table.table {
                tr {
                    td {("Price")} td{(listing.cost)}
                }
                tr {
                    td {("Currency")} td{(listing.currency)}
                }
            }
            ("Pay code: ") br;

            form.form-group method="POST" action="/make_payment" {
                input type="hidden" id="listing_id" value=(listing.id) name="listing_id";
                input.btn.btn-primary type=("submit") id="submit" value="Pay for this!";
            }

            div.qr_container { (PreEscaped(qr_data)) }
        },
    )
}
