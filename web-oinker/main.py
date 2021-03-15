from flask import Flask,render_template, send_from_directory,request,redirect
from os import listdir
from os.path import isfile, join
import subprocess

app = Flask(__name__)



oinks = {
    1:"testing oink",
    2:"utflag{traversal_bad_dude}",
    3:"do y'all like our logo?",
    4:"pog"
}

common = [
    "the",
    "be",
    "to",
    "of",
    "and",
    "a",
    "that",
    "have",
    "I",
    "it",
    "for",
    "not",
    "on",
    "with",
    "he",
    "as",
    "you",
    "do",
    "this",
    "is"

]

ind =  0
def is_png(f):
    return f.split(".")[1] == "png"

@app.route('/')
def uwu():
    return render_template("index.html")

@app.route('/oink/<path:path>')
def stonks(path):
    if int(path) not in oinks:
        return redirect("/")
    return render_template("oink.html",oink=oinks[int(path)])

@app.route('/postoink')
def postOink():
    global ind

    newind = (ind % 100) + 4
    oink = request.args["oink"]
    for i in common:
        oink = oink.replace(i,"oink")
    oinks[newind] = oink
    ind += 1
    return  redirect("/oink/" + str(newind))

@app.route('/imgs/<path:path>')
def get_img(path):
    return send_from_directory("./imgs/",path)

@app.route('/main.py')
def Pog():
    return send_from_directory('.','main.py')

if __name__ == '__main__':
    app.run(host='0.0.0.0')
