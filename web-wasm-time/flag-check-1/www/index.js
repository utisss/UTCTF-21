import * as wasm from "wasm-flag-check";

window.onload = () => {
    document.getElementById("flag_form").onsubmit = () => {
        const guess = document.getElementById("flag").value;
        
        if(wasm.check_flag(guess.split('').map(a => a.charCodeAt(0)))) {
            alert("You got the flag!");
        }else{
            alert("try again!");
        }

        return false;
    };
}

particlesJS.load('particles', 'particlesjs-config.json', function() {
    console.log('particles loaded');
})