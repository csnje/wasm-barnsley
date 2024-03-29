# Barnsley fern

## About

An implementation of the [Barnsley fern](https://en.wikipedia.org/wiki/Barnsley_fern) in **Rust** **WebAssembly**.

![Image of Barnsley fern](./images/output.png)

## Prerequisites

Install [**Rust**](https://www.rust-lang.org/) and [**wasm-pack**](https://github.com/rustwasm/wasm-pack).

## Build

```bash
wasm-pack build --target web
```
or optimised for release
```bash
wasm-pack build --target web --release
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
