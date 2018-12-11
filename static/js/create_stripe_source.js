var stripe;
var elements;

function stripeInit(pk) {
    stripe = Stripe(pk);
    elements = stripe.elements();
}

stripeInit("pk_test_dpoNF38dtaRvXrJQ1T4vDRcb");

window.onload = function() {
    var form = document.getElementById('payment-form');
    var card = elements.create('card');
    card.mount("#card-element");
    form.addEventListener('submit', function (event) {
        event.preventDefault();
        console.log("debug: submit event")
        var ownerInfo = {
            owner: {
                name: document.getElementById("name").value,
                address: {
                    line1: document.getElementById("addressLine1").value,
                    city: document.getElementById("city").value,
                    postal_code: document.getElementById("postCode").value,
                    country: document.getElementById("country").value,
                },
                email: document.getElementById("email").value
            },
        };

        stripe.createSource(card, ownerInfo).then(function (result) {
            if (result.error) {
                // Inform the user if there was an error
                var errorElement = document.getElementById('card-errors');
                errorElement.textContent = result.error.message;
            } else {
                // Send the source to your server
                console.log(stripe.source);
                stripeSourceHandler(result.source);
            }
        });
    });
};

function stripeSourceHandler(source) {
    // Insert the source ID into the form so it gets submitted to the server
    var form = document.getElementById('payment-form');
    var hiddenInput = document.createElement('input');
    hiddenInput.setAttribute('type', 'hidden');
    hiddenInput.setAttribute('name', 'stripeSource');
    hiddenInput.setAttribute('value', source.id);
    form.appendChild(hiddenInput);

    // Submit the form
    form.submit();
}