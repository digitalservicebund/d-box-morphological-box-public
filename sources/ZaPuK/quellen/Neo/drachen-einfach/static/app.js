var app = {
  destinationId: "bcdfdbf3-19aa-4c5a-a6ec-293ae203d4f7",
  schemaUri: "https://schema.fitko.de/fim/s00000092_1.0.schema.json",
  ars: "064350014014",
  leikaKey: "99123456760610",
  form: null,
  matrixID: '@user:matrix.org',
  uuid: '',
  sessionTimeout: 500,
  userData: {},
  comment: '' // Kommentar für die Antragsbearbeitung
};

function showPage(id) {
  const pages = document.querySelectorAll('.page.show');

  pages.forEach(page => {
    page.classList.remove('show');
  });

  document.querySelector(id).classList.add('show');

  // update URL
  const url = new URL(window.location.href);
  url.searchParams.set('page', id.replace('#page', ''));
  window.history.pushState({}, '', url.toString());

  // add form data
  if (id.replace('#page', '') == 'Form') {
    app.userData = {
      'firstname': 'Hildegard',
      'lastname': 'von Kunegundingen',
      'address': 'Im Verliesgraben 3'
    };
  }
}

function showPageWelcome() {
  showPage('#pageWelcome');
}

function showPageForm(data) {
  showPage('#pageForm');
  console.log(data);
  if ('user-data' in data) {
    app.userData = data['user-data'];
  }
}

function showPageFormWithDefaults() {
  showPageForm({
    'firstname': 'Hildegard',
    'lastname': 'von Kunegundingen',
    'address': 'Im Verliesgraben 3'
  });
}

function showPageQRCode() {
  showPage('#pageQRCode');
}

function showPageSuccess() {
  networkSendSubmission();
  showPage('#pageSuccess');
}

(() => {
  const url = new URL(window.location.href);

  // get destionationID from url param
  const destinationId = url.searchParams.get('destinationId');
  if (destinationId != undefined && destinationId != '') {
    localStorage.setItem('destinationId', destinationId);
  }
  console.log('destionationId:', localStorage.getItem('destinationId'));
  app.destinationId = localStorage.getItem('destinationId');

  // show page on load
  const page = url.searchParams.get('page');
  if (page != undefined && page != '') {
    showPage('#page' + page);
  }
})();

// initialize vue.js
var vm = new Vue({
  el: "#app",
  data: app,
  methods: {
  },
  computed: {
    locationHref: function () {
      return location.href;
    },
  },
});