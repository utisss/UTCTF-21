# Among Us Predictor
Among Us CTF
Web
Hard 900

From the source code we know that the flag is stored in `flag.txt`. We're able to upload images which are then visible on the site. By reading the source code you can notice that their is no path validations on the exec url. We are then able to craft a python script that reads the flag file and prints it to stdio. We can then call the script by visiting `isss.io:5000/exec/imgs/flag.png`. 
