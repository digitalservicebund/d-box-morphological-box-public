function generateAndShowBarcode(url) {
  var elem = document.getElementById('qrcode');
  elem.innerHTML = '';
  new QRCode(elem, url);
}
