from flask import Flask, render_template_string
import requests

app = Flask(__name__)

@app.route("/")
def home():
    res = requests.get("http://localhost:4000/stats")
    data = res.json()
    return render_template_string("""
        <h1>Server Stats</h1>
        <p>Users: {{ users }}</p>
        <p>Messages: {{ messages }}</p>
    """, **data)

if __name__ == "__main__":
    app.run(port=8000, debug=True)
