from flask import Flask, render_template, request, redirect, url_for, flash, make_response
from json import loads 

app = Flask(__name__)

@app.route('/', methods=['POST', 'GET'])
def game():
	if request.method == 'POST':
		# lol just assume they lost
		res = make_response('Tough luck, no flag for you. Better luck next time?')
		# get the score from the client
		new_score = loads(request.data.decode('utf-8'))
		# try to get the old high score from the cookie
		cookie_data = request.cookies.get('highScore')
		# two reasons to make a new cookie:
		# 1. there is no cookie yet
		# 2. the high score in the cookie needs to be updated
		if not cookie_data or new_score['score'] > int(cookie_data):
			res.set_cookie('highScore', str(new_score['score']), max_age=60*60*24)
			res.headers['location'] = url_for('game')
		
		try:
			# high score is either current score or val in cookie
			high_score = max(new_score['score'], int(cookie_data))
			if(high_score > 1000000):
				res.set_data("Congrats! The flag is utflag{numnum_cookies_r_yumyum}")
		finally:
			return res, 200
	return render_template('game.html')

if __name__ == "__main__":
	app.run(host='0.0.0.0')

