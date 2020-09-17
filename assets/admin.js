const dom_audio = document.querySelector("audio");
var lyric;

// Add audio handlers
dom_audio.onplay = function () {
  lyric.play(dom_audio.currentTime * 1000);
};
dom_audio.onpause = function () {
  lyric.pause();
};

// Update audio element file
const dom_titles = document.querySelectorAll("ul.titles li a");
dom_titles.forEach(
  (dom_title) =>
    (dom_title.onclick = function () {
      dom_audio.src = `media/${this.text}.mp3`;
      set_lyric();
      dom_audio.play();
    })
);

// Wait for the websocket handshake to succeed
ws.onopen = function () {
  lyric = new Lyric({
    onPlay: function (line, text) {
      ws.send(text);
      console.log(line, text);
    },
    onSetLyric: function (lines) {
      // console.log(lines)
      // ws.send(lines)
    },
  });
};

function set_lyric() {
  // LRC file has the same name as the audio file
  var lyric_url = dom_audio.src.replace(/\..+?$/, ".lrc");
  fetch(lyric_url).then((response) =>
    response.text().then(function (text) {
      lyric.setLyric(text);
    })
  );
}
