FROM golang:alpine AS gobuilder

FROM rust:alpine AS rustbuilder

COPY --from=gobuilder "/usr/local/go" "/usr/local/go"
ENV PATH /usr/local/go/bin:$PATH

# Build
WORKDIR /app
COPY . .

# add CC
RUN apk update && apk add musl-dev \
  && cargo build --release

# build a small image
FROM alpine

# Copy our static executable.
COPY --from=rustbuilder "/app/target/release/ffi-rust-go" "/bin/ffi-rust-go"

# RUN /bin/ffi-rust-go

# ENTRYPOINT ["/bin/ffi-rust-go"]
