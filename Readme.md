# Codepad


**Codepad** is an _efficient_ and _minimal_ open-source collaborative text
editor based on the operational transformation algorithm. It lets users
collaborate in real time while writing code in their browser. Codepad is
completely self-hosted and fits in a tiny Docker image, no database required.

The server is written in Rust using the
[warp](https://github.com/seanmonstar/warp) web server framework and the
[operational-transform](https://github.com/spebern/operational-transform-rs)
library. We use [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) to
compile text operation logic to WebAssembly code, which runs in the browser. The
frontend is written in TypeScript using [React](https://reactjs.org/) and
interfaces with [Monaco](https://github.com/microsoft/monaco-editor), the text
editor that powers VS Code.

Architecturally, client-side code communicates via WebSocket with a central
server that stores in-memory data structures. This makes the editor very fast,
allows us to avoid provisioning a database, and makes testing much easier. The
tradeoff is that documents are transient and lost between server restarts, or
after 24 hours of inactivity.

## Development setup

To run this application, you need to install Rust, `wasm-pack`, and Node.js.
Then, build the WebAssembly portion of the app:

```
wasm-pack build codepad-wasm
```

When that is complete, you can install dependencies for the frontend React
application:

```
npm install
```

Next, compile and run the backend web server:

```
cargo run
```

While the backend is running, open another shell and run the following command
to start the frontend portion.

```
npm run dev
```

This command will open a browser window to `http://localhost:5173`, with hot
reloading on changes.

## Testing

To run integration tests for the server, use the standard `cargo test` command.
For the WebAssembly component, you can run tests in a headless browser with

```
wasm-pack test --chrome --headless codepad-wasm
```

## Configuration

Although the default behavior of Codepad is to store documents solely in memory
and collect garbage after 24 hours of inactivity, this can be configured by
setting the appropriate variables. The application server looks for the
following environment variables on startup:

- `EXPIRY_DAYS`: An integer corresponding to the number of days that inactive
  documents are kept in memory before being garbage collected by the server
  (default 1 day).
- `SQLITE_URI`: A SQLite connection string used for persistence. If provided,
  Codepad will snapshot document contents to a local file, which enables them to
  be retained between server restarts and after their in-memory data structures
  expire. (When deploying a Docker container, this should point to the path of a
  mounted volume.)
- `PORT`: Which local port to listen for HTTP connections on (defaults to 3030).
- `RUST_LOG`: Directives that control application logging, see the
  [env_logger](https://docs.rs/env_logger/#enabling-logging) docs for more
  information.

## Deployment

Codepad is distributed as a single 6 MB Docker image, which is built
automatically from the `Dockerfile` in this repository. You can pull the latest
version of this image from Docker Hub. It has multi-platform support for
`linux/amd64` and `linux/arm64`.

```
docker pull sumitkumar17/codepad
```

(You can also manually build this image with `docker build -t codepad .` in the
project root directory.) To run locally, execute the following command, then
open `http://localhost:3030` in your browser.

```
docker run --rm -dp 3030:3030 codepad
```
<sup>
All code is licensed under the <a href="LICENSE">MIT license</a>.
</sup>
