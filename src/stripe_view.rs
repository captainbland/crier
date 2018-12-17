use maud::*;
use navbar_info::NavbarInfo;
use render_tools::*;
use validator::ValidationErrors;

pub fn render_payer_signup_form(navbar_info: &NavbarInfo, errors: &ValidationErrors) -> Markup {
    render_page_with_scripts("Register to pay with us", navbar_info,html!{
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
    }, vec!["https://js.stripe.com/v3/", "/static/js/create_stripe_source.js"])
}

pub fn render_create_listing_form(navbar_info: &NavbarInfo,  errors: &ValidationErrors) -> Markup {
    render_page("Create listing", navbar_info,html!{
        form.form-group method="POST" action="/create_listing" {
            (render_input("Title", "title", "text", errors))
            (render_input("Amount", "amount", "number", errors)) //FIXME: make this adjustable by step, extra attributes, etc.
            (render_input("Quantity", "quantity", "number", errors))
            input.btn.btn-primary type=("submit") value="Register";
        }
    })

}