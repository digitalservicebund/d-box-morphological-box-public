var networkTimerStart = 1000;
var networkTimerMax = 10000;

function setSessionProgress() {
  var elem = document.querySelector('#sessionProgress');
  var colors = elem.classList;

  if (app.sessionTimeout === networkTimerStart) {
    elem.innerHTML = '';

    colors.remove('is-warning');
    colors.remove('is-danger');
    colors.add('is-primary');
    colors.add('is-loading')
    colors.add('is-large')
  }

  if (app.sessionTimeout > 2000) {
    colors.remove('is-primary');
    colors.add('is-warning');
  }

  if (app.sessionTimeout > 5000) {
    colors.remove('is-warning');
    colors.add('is-danger');
  }

  if (app.sessionTimeout > networkTimerMax) {
    colors.remove('is-loading');
    colors.remove('is-large')
    elem.innerHTML = 'Das hat leider nicht geklappt';
  }
}

function resetSessionProgress() {
  app.sessionTimeout = networkTimerStart;

  //setSessionProgress();
}

function networkPollSession() {
  window.setTimeout(pollSessionCallback, app.sessionTimeout);
}

function networkPollOnError() {
  //app.sessionTimeout = app.sessionTimeout * 1.25
  //setSessionProgress();

  if (app.sessionTimeout > 10000) {
    //return;
  }

  networkPollSession();
}

function pollSessionCallback() {
  const serverHost = '';
  const serverHost2 = 'https://fuehrungszeugnis.fitko.dev';
  const apiSession = '/api/v1/sessions/' + app.uuid;
  const server = serverHost + apiSession;

  fetch(server)
  .then((response) => response.json())
  .then(data => {
    if (data && data.error) {
      networkPollOnError();

      return;
    }

    // console.log(data);

    showPageForm(data);
  }).catch(error => {
    networkPollOnError();
  });

  /*
   Antrag am OD verschicken: echo '{"destinationId":"2fc884cb-e5a5-4631-aa55-67b18d52b548", "leikaKey":"urn:de:fim:leika:leistung:99049001001003"}' | http post http://127.0.0.1:5000/api/v1/submit-form --json
  */
 /**
  * echo '{"hallo":"welt"}'| http put http://127.0.0.1:5000/api/v1/sessions/9c9dd4fd-2018-47fc-875e-e1a4e44ffb03 --json
  */
}

function networkSendSubmission() {
  app.userData['comment'] = app.comment;
  app.userData['$schema'] = "https://schema.fitko.de/fim/s00000000_1.0.schema.json";

  console.log(app.destinationId);

  fetch('/api/v1/submit-form', {
    method: 'POST',
    body: JSON.stringify({
      "destinationId": app.destinationId,
      "leikaKey": "urn:de:fim:leika:leistung:99049001001003",
      "data": app.userData
    }),
    headers: {
    'Content-Type': 'application/json'
    }
  })
  .then(res => res.json())
  .then(res => {
    console.log(res);
  }).catch(error => {
    alert("Fehler beim Senden des Antrags!");
  });

}
