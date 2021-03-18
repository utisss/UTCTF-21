let canvas = document.getElementById("myCanvas");
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;
let c = canvas.getContext("2d");

//define constants for cookie
let cookie = {
    radius: canvas.width < canvas.height ? canvas.width/3 : canvas.height/3,
    x: canvas.width/2,
    y: canvas.height/2,
    base_color: '#DEB887',
    choco_color: '#401801',
    face_color: '#261212',
    blush_color: '#D99C79',
};

//define game vars
let numClicks = 0;
let playingGame = true;
//see if you can get the player's high score from the cookies
let highScore = "0";
let cookies = decodeURIComponent(document.cookie).split(';');
for(let i = 0; i < cookies.length; i++){
    if(cookies[i].includes("highScore"))
        highScore = cookies[i].substring(cookies[i].indexOf("=") + 1);
}

function pressedCookie(mouseX, mouseY){
    //how far is the mouse from the center of the cookie?
    dx = mouseX - cookie.x;
    dy = mouseY - cookie.y;
    //is it within the cookie radius?
    return (dx * dx) + (dy * dy) <= cookie.radius * cookie.radius;
}

//event listener for if canvas clicked
window.addEventListener('click', function(event){
    if(pressedCookie(event.x, event.y) && playingGame){
        numClicks++;
    }
})

//event listener for when browser window resized
window.addEventListener('resize', function(){
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    cookie.radius = canvas.width < canvas.height ? canvas.width/3 : canvas.height/3;
    cookie.x = canvas.width/2;
    cookie.y = canvas.height/2;
})

function drawCirc(x, y, rad, color){
    c.beginPath();
    c.arc(x, y, rad, 0, Math.PI *2, true);
    c.fillStyle = color;
    c.fill();
}

function drawCookie(time) {
    c.clearRect(0, 0, canvas.width, canvas.height);
    let x = cookie.x;
    let y = cookie.y;
    let radius = cookie.radius;
    // draw main cookie body
    drawCirc(x, y, radius, cookie.base_color);

    // draw a few chocolate chunks
    drawCirc(x - 3*radius/5, y + radius/3, radius/5, cookie.choco_color);
    drawCirc(x - radius/2, y - 2*radius/5, radius/4, cookie.choco_color);
    drawCirc(x, y - 4*radius/5, radius/9, cookie.choco_color);
    drawCirc(x + radius/10, y + 3*radius/5, radius/5, cookie.choco_color);
    drawCirc(x + radius/2, y - radius/2, radius/4, cookie.choco_color);
    drawCirc(x + 2*radius/3, y + radius/3, radius/10, cookie.choco_color);

    //draw the blush
    drawCirc(x - radius/3, y, radius/10, cookie.blush_color);
    drawCirc(x + radius/3, y, radius/10, cookie.blush_color);

    // draw the face
    c.strokeStyle = cookie.face_color;
    c.lineWidth = 3;
    c.beginPath(); //left eye
    c.arc(x - radius/6, y - radius/12, radius/14, 0, Math.PI, true);
    c.stroke();
    c.beginPath(); //right eye
    c.arc(x + radius/6, y - radius/12, radius/14, 0, Math.PI, true);
    c.stroke();
    c.beginPath(); //smile
    c.arc(x, y, radius/16, 0, Math.PI, false);
    c.stroke();

    display_score(time);
}

function display_score(time){
    c.font = "32px Arial";
    c.fillStyle = cookie.face_color;
    c.textAlign = "start";
    c.fillText("Your best: " + highScore, canvas.width/10, 9*canvas.height/10, canvas.width);
    c.fillText("Your score: " + numClicks, canvas.width/10, canvas.height/10, canvas.width);
    c.textAlign = "end";
    c.fillText("Time remaining: " + (30 - Math.min(parseInt(time/1000), 30)), 9*canvas.width/10, canvas.height/10, canvas.width);
    c.fillText("High score: 1,000,000", 9*canvas.width/10, 9*canvas.height/10, canvas.width);
}

function notify(text){
    if(confirm(text))
        location.reload();
}

function timer(curTime){
    requestAnimationFrame(timer);
    drawCookie(curTime);
    if(curTime > 30000 && playingGame){
        playingGame = false;
        //CB and alter url before using on isss server
        fetch("http://" + location.hostname + ":4270",{
            method: "POST",
            credentials: 'same-origin',
            body: JSON.stringify({
                score: numClicks,
            })
        })
        .then(response => response.text()).then(data => notify(data));
    }
}
  
requestAnimationFrame(timer);
