FROM rust:1.59.0-slim-bullseye as builder

WORKDIR /app

RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl

# Copy code
COPY ./Cargo.* ./
COPY ./src/ ./src/

# Compile
RUN ["cargo", "build", "--release", "--target", "x86_64-unknown-linux-musl", "-Z", "unstable-options", "--out-dir", "/app/bin"]

## Runtime image
FROM scratch

COPY --from=builder /app/bin/rusty-app /app

ENTRYPOINT ["/app"]

# vim:ft=dockerfile
