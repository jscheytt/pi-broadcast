# pi-broadcast

[![Build status](https://github.com/jscheytt/pi-broadcast/workflows/build/badge.svg)](https://github.com/jscheytt/pi-broadcast/actions?query=workflow%3Abuild)
[![Release status](https://github.com/jscheytt/pi-broadcast/workflows/release/badge.svg)](https://github.com/jscheytt/pi-broadcast/actions?query=workflow%3Arelease)
[![semantic-release](https://img.shields.io/badge/%20%20%F0%9F%93%A6%F0%9F%9A%80-semantic--release-e10079.svg)](https://github.com/semantic-release/semantic-release)
[![Commitizen friendly](https://img.shields.io/badge/commitizen-friendly-brightgreen.svg)](http://commitizen.github.io/cz-cli/)

This is a little web server that facilitates home church services for small groups.
It enables you to play a song and stream the song lyrics live to everybody's mobile device
This can be especially helpful when you have nobody available who can lead worship with an instrument.

## Setup

### Prerequisites

* Raspberry Pi (>= 2)
* Newest release (latest item in [Releases](https://github.com/jscheytt/pi-broadcast/releases))
* MP3s with corresponding LRC files

### Creating media

This application uses MP3 files combined with LRC files to enable displaying live lyrics.
The LRC format is well described on [Wikipedia](https://en.wikipedia.org/wiki/LRC_(file_format)).

If you have an MP3 file and want to create lyrics, have a look at the sample file in the `media/` directory:

1. The LRC file has the *same name* as the corresponding audio file (apart from the extension of course).
1. Every lyric **segment** is on a separate line.
1. Every segment starts with a timestamp.
1. If you want your segment to contain text that spans multiple lines, you have to separate each line with a `<br>` tag (just as in typical HTML).
1. Additional tags like the `[length:xxx]` are not required for the streaming to work.

## Usage

The release package contains a binary application (the server) and some static assets (the webpages and some scripts).
This means you don't have to install any additional software: Just download, unzip, run it!

1. Get the release unto your Raspberry. Some options for doing this:
    * Download it directly on the Raspberry.
    * Download it on another machine, put it on a USB drive and plug it in. This can make media management easier (e. g. if you want to add new media often, it might be more convenient to do it on a different machine than the Pi).
1. Unzip it.
    * We will call the path to which you unpackaged the release the **target directory**.
1. Add media (see [Creating media](#creating-media)) into the `media/` directory.
1. Start the server: You will have to run `sudo ./pi-broadcast` in a terminal. (Double-clicking on the executable will not work because running on port 80 seems to require admin privileges.) You have some options for this:
    1. Manually:
        1. Open a terminal.
        1. Navigate to the target directory.
        1. Execute the above command.
    1. With shortcut:
        1. Copy the `.desktop` file from the `assets/` directory to the Raspberry's desktop
        1. Adjust the file path: Edit the `.desktop` file and change the `Path=` entry to the file path where the executable is.
        1. Double-click on the shortcut.
1. Find out your Raspberry's IP address. You can do this e. g. by opening a terminal and executing `hostname -I`
    * The output should be something like `192.168.0.38 2a02:8070:8997:b700:9f94:3499:ae01:e3b2`.
    * The first part before the space is the IP address of your Raspberry.
    * We will reference the IP address with `{ip_address}` in the following steps.
1. Connect to the network in which your Raspberry is running with a device of your choice (e. g. a tablet)
1. Visit http://{ip_address}/admin (the admin view)
1. Connect to Raspberry's network with another device (e. g. a smartphone).
1. Visit http://{ip_address} (the "homepage")
1. On the first device (in this case: tablet): Play one of the songs by clicking on them.
1. On the second device (in this case: smartphone): Watch the lyrics display.

## Development

See [Contributing Guidelines](CONTRIBUTING.md)

## CI/CD

GitHub Actions performs the following when you merge changes to master:

1. Perform a semantic-release step. This determines the next version number from the commit messages.
1. If there is a new version number (= a new release):
    1. Update changelog.
    1. Create new release.
    1. Compile binary for target platform.
    1. Create zip and attach it to release.

This means you will never have to create a release manually.
