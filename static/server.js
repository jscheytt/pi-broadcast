const send = document.querySelector("button#send");

ws.onopen = function() {
  send.onclick = function() {
      const msg = new Date();
      ws.send(msg);
  };
};
