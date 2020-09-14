const send = document.querySelector('button#send');

// Wait for the websocket handshake to succeed
ws.onopen = function() {
  // Now define handler for sending messages
  send.onclick = function() {
      const message = new Date();
      ws.send(message);
  };
};
