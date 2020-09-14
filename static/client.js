const target = document.querySelector("article");

function message(data) {
  const line = document.createElement('p');
  line.innerText = data;
  target.appendChild(line);
}

ws.onopen = function() {
  target.innerHTML = '<p><em>Connected!</em></p>';
};

ws.onmessage = function(msg) {
  console.log("message arrived");
  message(msg.data);
};

ws.onclose = function() {
  target.innerHTML = '<p><em>Disconnected!</em></p>';
};
