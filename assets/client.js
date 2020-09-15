const target = document.querySelector('article');

ws.onopen = function() {
  target.innerHTML = '<p><em>Connected!</em></p>';
};

ws.onmessage = function(message) {
  console.debug('Message arrived');
  target.innerHTML = '<p>' + message.data + '</p>';
};

ws.onclose = function() {
  target.innerHTML = '<p><em>Disconnected!</em></p>';
};
