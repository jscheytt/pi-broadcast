# Contributing Guidelines

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
