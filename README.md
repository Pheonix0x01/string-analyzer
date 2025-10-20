# String Analyzer Service

A RESTful API service built with Rust and Actix-web that analyzes strings and computes various properties including length, palindrome detection, character frequency, and SHA-256 hashing.

## Features

- Analyze strings and compute 6 different properties
- Store analyzed strings with unique SHA-256 identification
- Filter strings by multiple criteria
- Natural language query support
- RESTful API with proper HTTP status codes


## Prerequisites

- Rust 1.80 or higher
- Cargo

## Installation

Clone the repository:

```bash
git clone <the repo url>
cd string_analyzer
```

Install dependencies:

```bash
cargo build
```

## Running Locally

Start the development server:

```bash
cargo run
```

The server will start on `http://localhost:8080`

For production build:

```bash
cargo build --release
./target/release/string_analyzer_service
```

## Environment Variables

Create a `.env` file in the project root:

```env
PORT=8080
LOG_LEVEL=info
RUST_LOG=info
```