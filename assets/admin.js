const dom_audio = document.querySelector("audio");
var lyric, current_title;

// Update audio element file
const dom_titles = document.querySelectorAll("ul.titles li a");
dom_titles.forEach(
  (dom_title) =>
    (dom_title.onclick = function () {
      current_title = this.text;
      dom_audio.src = `media/${this.text}.mp3`;
      set_lyric();
      dom_audio.play();
    })
);

// Wait for the websocket handshake to succeed
ws.onopen = function () {
  lyric = new Lyric({ onPlay: dispatch_lyric });

  // Add audio handlers
  dom_audio.onplay = function () {
    lyric.play(dom_audio.currentTime * 1000);
  };
  dom_audio.onpause = function () {
    lyric.pause();
  };
};

function dispatch_lyric(line, text) {
  console.debug(line, text);
  // Based on the thoughtbot convention from https://thoughtbot.com/blog/json-event-based-convention-websockets
  event_name = "lyric_sent";
  event_data = {
    title: current_title,
    lyric: text,
    line: line,
  };
  data = JSON.stringify({ event: event_name, data: event_data });
  ws.send(data);
}

function set_lyric() {
  // LRC file has the same name as the audio file
  var lyric_url = dom_audio.src.replace(/\..+?$/, ".lrc");
  fetch(lyric_url).then((response) =>
    response.text().then(function (text) {
      lyric.setLyric(text);
    })
  );
}
