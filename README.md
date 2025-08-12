# wasm-barnsley

## About

An implementation in **Rust** **WebAssembly** and **JavaScript** of the [Barnsley fern](https://en.wikipedia.org/wiki/Barnsley_fern).

![Image of Barnsley fern](./images/output.png)

## Prerequisites

A [**Rust**](https://www.rust-lang.org/) installation.

## Build

```sh
cargo build --target=wasm32-unknown-unknown --release
```

## Run

Some options to serve the application include:
```bash
# Python 3.x
python3 -m http.server
# Python 2.x
python -m SimpleHTTPServer
# JDK 18 or later
jwebserver
```

Access via a web browser at [http://localhost:8000](http://localhost:8000).
