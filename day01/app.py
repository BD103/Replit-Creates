# You may be asking why I used Flask when this can staticly host?
# Well because Replit doesn't like 404.html files. :/

from flask import Flask, send_file, redirect


app = Flask(__name__)


@app.route("/")
def index_redirect():
    return redirect("error_404")


@app.route("/lNDEX.htm")
def index():
    return send_file("lNDEX.htm")


@app.errorhandler(404)
def error_404(error):
    return send_file("404.htm"), 404
