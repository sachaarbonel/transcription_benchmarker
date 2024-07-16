
# Multipart Request Benchmarking Tool

This is a command-line tool for benchmarking HTTP endpoints, with support for both JSON and multipart requests. It's particularly useful for testing APIs that handle file uploads or audio processing.

## Features

- Support for both JSON and multipart requests
- Concurrent request handling
- Latency measurement for each request
- Summary statistics (average, min, max latency)
- Customizable number of requests and concurrency level
- Flexible header and data input

## Installation

Ensure you have Rust and Cargo installed on your system. Then, clone this repository and build the project:

```bash
git clone https://github.com/yourusername/multipart-request-benchmarker.git
cd multipart-request-benchmarker
cargo build --release
```

The executable will be available in `target/release/`.

## Usage

Here are some example commands:

1. For a JSON request:

```bash
cargo run -- -n 10 -c 5 -m POST -H "Content-Type: application/json" -d '{"language":"en","audio_url":"http://example.com/jfk.wav"}' http://localhost:3000/transcribe
```

2. For a multipart request with file upload:

```bash
cargo run -- -n 10 -c 5 -m POST -d '{"language":"en"}' -f path/to/audio.wav http://localhost:3000/transcribe
```

for whisper.cpp
```bash
cargo run -- -n 10 -c 5 -m POST --audio-part-name file -d '{"language":"en"}' -f path/to/audio.wav http://localhost:8080/inference
```

### Command-line Options

- `-n, --num-requests <NUM>`: Number of requests to send (default: 1)
- `-c, --concurrency <NUM>`: Number of concurrent requests (default: 1)
- `-m, --method <METHOD>`: HTTP method to use (default: POST)
- `-H, --headers <HEADERS>`: HTTP headers (can be used multiple times)
- `-d, --data <DATA>`: Request body data
- `-f, --file <FILE>`: File to upload (for multipart requests)
- `<URL>`: Target URL (required)

## Output

The tool will print the status, headers, and body of each response, along with the latency. After all requests are completed, it will display summary statistics including the total number of requests, average latency, and min/max latencies.

## Dependencies

- clap: For parsing command-line arguments
- reqwest: For making HTTP requests
- tokio: For asynchronous runtime
- futures: For working with asynchronous code
- tokio-util: For utilities like codecs
- anyhow: For error handling

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
