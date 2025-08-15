# Gatespy

Gatespy is a simple **port scanner** written in **Rust**.  
This project is intended as a **learning exercise** for Rust, focusing on ownership, error handling, and basic networking concepts.  

## Features

- Scan ports 1 to 1024 sequentially
- Command-line interface (CLI)
- Lightweight and easy to understand
- Designed as a beginner-friendly Rust project

## Prerequisites

- Rust (>=1.70)
- Cargo (comes with Rust)

## Installation

Clone the repository:

```bash
    git clone https://github.com/your-username/gatespy.git
    cd gatespy
```

Build the project:
```bash
    cargo build
```

Or run directly with Cargo:
```bash
    cargo run -- <host>
```

Example:
```bash
    cargo run -- 127.0.0.1
```

## Usage

- Pass a host as a command-line argument
- The scanner will try to connect to ports 1 through 1024
- Open ports will be printed in the console

## Contributing
This project is intended as a learning exercise, but contributions and suggestions are welcome.