from flask import Flask, render_template, request, redirect
import requests

app = Flask(__name__)

@app.route("/")
def home():
    return redirect("/login")

@app.route("/login", methods=["GET", "POST"])
def login():
    if request.method == "GET":
        return render_template("login.html")

    data = {"username": request.form.get("username"), "password": request.form.get("password")}
    
    try :
        response = requests.post("http://127.0.0.1:4000/verify", json=data)
    except :
        return "Err checking microservice unresponsive/malfunctioning"
        
    print("Status code:", response.status_code)
    print("Raw text:", response.text)  # <-- see what came back
    result = response.json()
    if result.get("known"):
        return redirect("/dashboard")
    return render_template("login.html")
    

@app.route("/dashboard")
def dashboard():
    return render_template("dashboard.html")

if __name__ == "__main__":
    app.run(port=8000, debug=True)
