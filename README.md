# Rust Webserver

This project is a simple multi-threaded web server written in Rust. It demonstrates how to handle HTTP requests concurrently using a custom thread pool implementation.

## Features
- Serves static HTML files (`hello.html`, `404.html`)
- Handles basic GET requests
- Demonstrates thread pool usage for concurrent request handling
- Graceful shutdown after handling a set number of requests

## Project Structure
- `src/lib.rs`: Contains the implementation of the `ThreadPool` and `Worker` structs.
- `src/bin/main.rs`: Entry point for the web server. Sets up the TCP listener and handles incoming connections.
- `hello.html`: HTML file served for successful requests.
- `404.html`: HTML file served for not found requests.
- `Cargo.toml`: Project configuration and dependencies.

## How It Works
1. The server listens on `127.0.0.1:7878`.
2. For each incoming connection, a job is sent to the thread pool for processing.
3. The request is parsed, and the appropriate HTML file is served based on the request path.
4. The server demonstrates handling a `/sleep` route by sleeping for 10 seconds before responding.
5. After handling two requests, the server shuts down gracefully.

## Running the Server

1. Build the project:
   ```sh
   cargo build --release
   ```
2. Run the server:
   ```sh
   cargo run --bin main
   ```
3. Open your browser and navigate to [http://127.0.0.1:7878](http://127.0.0.1:7878)

## Example Requests
- `GET /` returns `hello.html`
- `GET /sleep` waits 10 seconds, then returns `hello.html`
- Any other path returns `404.html`

## License
This project is licensed under the MIT License.
