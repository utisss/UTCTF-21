 # Cutest Cookie Clicker Rip-Off
 * **Event:** UTCTF
 * **Problem Type:** Web
 * **Point Value / Difficulty:** Easy (200 pts)
 * **(Optional) Tools Required / Used:** 
 ## Solution
 ### Option 1
 Maybe it's the heavy cookie theme. Maybe you looked at the source code and saw that it used cookies to keep track of the high score. Maybe you just fiddled around with developer tools til you found something interesting. Whatever the case, you eventually notice that there is a browser cookie called "highScore" keeping track of your best score. If you change the value to be greater than the high score you have to beat...viola! You've hacked the game and the server will reward you with the flag!

 ### Option 2
 Another option is to edit the client-side code. All the logic for the game is in the game.js file, so if you look around, you'll find the code that updates your score every time you click the cookie. You can change how much it increments the score per click, ctrl+S, and easily gain an absurdly high score.
 
 ### Option 3
 Yet another option is to modify the post request to the server. Once time is up, the game sends a post request to the server with your score. Modifying the body of the post request will allow you to lie to the server and give it a different, higher score.
