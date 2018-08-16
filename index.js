import { calculate_valid_words } from "./wordscapes";

function processForm(e) {
    if (e.preventDefault) e.preventDefault();

    console.log(calculate_valid_words(e.target[0].value));
    return false;
}

var form = document.getElementById('form-id');
if (form.attachEvent) {
    form.attachEvent("submit", processForm);
} else {
    form.addEventListener("submit", processForm);
}
