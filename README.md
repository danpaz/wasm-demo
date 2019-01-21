# wasm-demo

Demo of rust compiled to web assembly.

![demo](demo.gif)

## Setup

Based on https://www.hellorust.com/demos/add/index.html.

see & run ./cargo-build.sh

## Run
    python3 -m http.server 8000 -d public/
    python2 -m SimpleHTTPServer 8000 -d public/

Navigate to http://localhost:8000/public/index.html
