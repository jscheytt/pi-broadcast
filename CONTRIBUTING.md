# Contributing Guidelines

## How to write commit messages

This repository is based on [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).

**Before you start committing to this repository, please read the above link and learn writing Conventional Commits.**

We have included [commitizen](https://commitizen.github.io/cz-cli/#using-the-command-line-tool) and [commitlint](https://commitlint.js.org/#/) in the package.json so that you can have helpful tools that automatically guide you towards writing Conventional Commits.

## Good first issues

* Add a view for broadcasting arbitrary text e. g. if someone wants to share a text passage
    * You could move the "admin" view to a "play" view
    * and create a new "publish" view with a textbox and a send button
    * and some JS functionality to publish a JSON websocket message
* Add a view for broadcasting an image
    * You could reuse the "publish" view and add an upload button
    * You could publish the image via JSON as base64-encoded string
    * and decode it accordingly in the "home" view
* Translate all views

## Prerequisites

* [Rust](https://www.rust-lang.org/tools/install)
* [Node.js](https://nodejs.org/)

## Setup

1. Install prerequisites.
1. Install rust crates:  
`cargo build`
1. Install npm packages:  
`npm install`

## Run

The script has one CLI parameter: the HTTP port.
The default value is 80.

Running on port 80 seems to require admin privileges on most systems, so when you are developing better choose a different port.

Run the server e. g. under port 3030:

```sh
cargo run 3030
```

You can then open the application: http://localhost:3030/

## Dependencies

### Rust

Nothing special here.

### npm

The npm dependencies are only relevant for the semantic-release process in CI/CD.

### Client-side scripts

The scripts included in the HTML files/templates are completely local, none of them point to public URLs.
This has the great advantage that this server can run in a Raspberry hotspot without need for internet access.

So if you want to add a client-side script, write the code yourself (if that's a viable option) or add the full source code to the repo (see `assets/lrc-file-parser.min.js` as an example).
