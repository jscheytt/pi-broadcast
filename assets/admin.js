const dom_audio = document.querySelector('audio');
// LRC file has the same name as the audio file
const lyric_url = dom_audio.querySelector('source').src.replace(/\..+?$/,  '.lrc');

// Wait for the websocket handshake to succeed
ws.onopen = function() {
  const lyric = new Lyric({
    onPlay: function(line, text) {
      ws.send(text);
      console.log(line, text);
    },
    onSetLyric: function(lines) {
      // console.log(lines);
      // ws.send(lines);
    }
  });

  fetch(lyric_url)
    .then((response) => response.text()
      .then(function(text) {
        lyric.setLyric(text);
      }
    ));

  // Add audio handlers
  dom_audio.onplay = function() {
    lyric.play(dom_audio.currentTime * 1000);
  };
  dom_audio.onpause = function() {
    lyric.pause();
  };
};
