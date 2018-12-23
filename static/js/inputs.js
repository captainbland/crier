$(document).ready(function () {
   $(".currency").blur(function() {
        to_fixed(this);
   });
});

function to_fixed(elem) {
    console.log("Received element tofixed: ", elem)
    $(elem).val(parseFloat($(elem).val()).toFixed(2));
}