const target = document.querySelector('article');

ws.onopen = function() {
  target.innerHTML = '<p><em>Connected!</em></p>';
};

ws.onmessage = function(message) {
  console.info("Message arrived");
  const line = document.createElement('p');
  line.innerText = message.data;
  target.parentNode.replaceChild(line, target)
};

ws.onclose = function() {
  target.innerHTML = '<p><em>Disconnected!</em></p>';
};
