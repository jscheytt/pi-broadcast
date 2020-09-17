const dom_title = document.querySelector("h1");
const dom_lyric = document.querySelector("article p");

ws.onopen = function () {
  console.debug("Connected to Websocket");
};

ws.onclose = function () {
  console.debug("Disconnected from Websocket");
};

ws.onmessage = function (event) {
  console.debug("Message arrived");
  var data = JSON.parse(event.data).data;
  dom_lyric.innerHTML = data.lyric;
  dom_title.innerHTML = data.title;
};
