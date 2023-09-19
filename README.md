# FFI Rust/Go

This project implements a simple FFI call of Rust library in Go:

The Rust project is compiled as a static library
The Go project embeds the Rust library and accesses it with CGO

---

## Pre-requisites
**Step 1: Install Golang**

- Install a [correctly configured](https://golang.org/doc/install) Go toolchain (version 1.21+). 
- Make sure that your `GOPATH` and `GOBIN` environment variables are properly set up.

**Step 2: Install Rust**

- Install a [correctly configured](https://www.rust-lang.org/learn/get-started) Rust toolchain (version 1.58.0+). 

## Download source code:
```bash
# Download sources from github
git clone https://github.com/oligami-0424/ffi-rust-go

# Go to sources directory
cd ffi-rust-go
```

## Build and run binary:
```bash
# Build
cargo build

# Run
cargo run

## Build and run Docker container:
```bash
# Build Docker image
docker build -t ffi-rust-go .

# Run Docker container
docker run -it ffi-rust-go /bin/ash
/# ffi-rust-go
:print on rust
Segmentation fault
```

# Run
We have confirmed that it works fine in WSL (Ubuntu), but it does not work well in other operating systems.
## On WSL (Ubuntu)
Complete program!

## On alpine
Go into a Go function and cause a Segmentation fault.

## On windows
The moment I call a Go function, it freezes.

# I want to move everything normally. Is there any way to do this?
Your confirmation on the OS and input is welcome!
