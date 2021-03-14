# Source it!
* **Event:** UTCTF
* **Problem Type:** Web
* **Point Value / Difficulty:** Easy
* **(Optional) Tools Required / Used:**

## Steps
#### Step 1
Once you have navigated to the page, right click and view source. 

#### Step 2
You'll see an interesting form of security. The function takes in a user name and password, using a md5 hash to encrypt the password you enter and then compares it to the username, admin, and hashed password value the function keeps as variables. Not the best way to handle authentication.

#### Step 3
Take the md5 hash and crack it. There are a few ways to do this such using tools like hashcat or john the ripper, but the easiest is problably to just google search for a md5 hash cracker. Once cracked the revealed password is sherlock.

#### Step 4
Enter the username and password you found and a text box will pop up with the flag!

#### Step 3 - Alternative
You might have noticed the link to assets/js/main.js. If you were to do a little snooping (clicking the link) you'll see the flag right away and skip the cracking processing entirely!

### Flag
'utflag{b33n_th3r3_s0uRc3d_th4t}'
