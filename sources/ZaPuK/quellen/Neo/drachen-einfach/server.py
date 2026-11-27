from flask import Flask, request, abort, session
from fitconnect import FITConnectClient, Environment
from strictyaml import load
from datetime import datetime
import requests

# read config
with open('config.yaml') as file:
    config = load(file.read()).data

# initialize SDK
fitc = FITConnectClient(Environment[config['sdk']['environment']], config['sdk']['client_id'], config['sdk']['client_secret'])

app = Flask(__name__)
app.config['SECRET_KEY'] = config['flask_secret_key']

# success response
RESPONSE_SUCCESS = {
    'status': 'ok'
}

sessions = {}

ROUTING_API_URL = 'https://routing-api-testing.fit-connect.fitko.dev/v1'

@app.route('/api/v1/destinations/<destination_id>')
def get_destination(destination_id):
    print("destination", destination_id)
    r = fitc.get_destination(destination_id)
    if r.status_code == 404:
        return {
            'status': 'not_found'
        }
    elif r.status_code != 200:
        return {
            'status': 'unknown_error'
        }

    return {
        'status': 'ok',
        'destination': r.json()
    }

@app.route('/api/v1/routes')
def get_routes():
    ars = request.args.get('ars')
    leika_key = request.args.get('leikaKey')

    r = requests.get(ROUTING_API_URL + f'/routes?ars={ars}&leikaKey={leika_key}')
    return r.json()

@app.route('/api/v1/submit-form', methods = ['POST'])
def submit_form():
    submission_data = request.get_json()
    print(submission_data)
    status = fitc.submission(submission_data['destinationId'], submission_data['leikaKey'], data=submission_data['data'])
    return status

@app.route('/api/v1/sessions/<uuid:session_id>', methods = ['PUT'])
def put_session(session_id):
    global sessions
    sessions[session_id] = request.get_json()
    return(str(session_id))

@app.route('/api/v1/sessions/<uuid:session_id>')
def get_session(session_id):
    global sessions
    if session_id not in sessions:
        return {"error": "session_not_found"}
    return(sessions[session_id])

@app.route('/')
def index():
    return app.send_static_file('index.html')

@app.route('/bundid')
def bundid():
    return app.send_static_file('bundid.html')
