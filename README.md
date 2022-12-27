# Barnsley fern

## About

An implementation of the [Barnsley fern](https://en.wikipedia.org/wiki/Barnsley_fern) in Rust WebAssembly.

![Image of Barnsley fern](./images/output.png)

## Prerequisites

Install [wasm-pack](https://github.com/rustwasm/wasm-pack).

## Compile

```bash
wasm-pack build --target web
```

## Serve and run

```bash
# Python 2.x
python -m SimpleHTTPServer
# Python 3.x
python3 -m http.server
```

Run in a browser at [localhost:8000](localhost:8000).
