# Lyrics Streaming

## Usage

1. `cargo run`
1. Open http://localhost:3030/admin in one browser and http://localhost:3030 in a different browser
1. Start the audio in the admin view
1. The homepage view should display the current lyrics

## Create an LRC file for an audio file

The LRC format is well described on [Wikipedia](https://en.wikipedia.org/wiki/LRC_(file_format)).

Take a look at the sample file in the media directory:

1. The LRC file has the same name as the corresponding audio file.
1. Every lyric **segment** is on a separate line.
1. Every segment starts with a timestamp.
1. If you want your segment to contain text that spans multiple lines, you have to separate each line with a `<br>` tag.
1. Additional tags like the `[length:xxx]` are not required for the streaming to work.

## To Do

- [ ] Show file list (probably with templating on warp-side)
- [ ] Show file name in homepage as song title
