const target = document.querySelector("article p");

ws.onopen = function () {
  console.debug("Connected to Websocket");
};

ws.onclose = function () {
  console.debug("Disconnected from Websocket");
};

ws.onmessage = function (message) {
  console.debug("Message arrived");
  target.innerHTML = message.data;
};
