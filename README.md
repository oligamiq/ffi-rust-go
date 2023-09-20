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
on all
Complete program!

## On alpine
### on all
Go into a Go function and cause a Segmentation fault.

## On windows
The moment I call a Go function, it freezes.

### on nightly
error: linking with `link.exe` failed: exit code: 1120
libstd-d9ee307034db292c.rlib(std-d9ee307034db292c.std.e9da0a539f4e1ec4-cgu.0.rcgu.o) : warning LNK4078: multiple '.drectve' sections found with different attributes (00100A00)
          libffi_go_print.lib(000005.o) : error LNK2019: unresolved external symbol fprintf referenced in function _cgo_beginthread
          C:\Users\oligami\ffi-rust-go\target\debug\deps\ffi_rust_go.exe : fatal error LNK1120: 1 unresolved externals

# I want to move everything normally. Is there any way to do this?
Your confirmation on the OS and input is welcome!

# information
my test on https://github.com/oligami-0424/ffi-rust-go/tree/74ff80108e54d2c53252e241a8225859c45ed4f4

compile tool is gcc (Strawberry)

- |success| c lang call static golang
- |success| c lang call static c lang 
- |success| rust lang call static c lang 
- |failed | rust lang call static go lang 
