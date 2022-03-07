FROM rust:1.59.0-slim-bullseye

# Don't worry about all the x86_64-unknown-linux-musl stuff. It is necessary
# for a later exercise in order to compile the executable generically.
RUN rustup target add x86_64-unknown-linux-musl

# Copy code
COPY ./Cargo.* ./
COPY ./src/ ./src/

# Compile
RUN cargo build --release --target x86_64-unknown-linux-musl

ENTRYPOINT ["target/x86_64-unknown-linux-musl/release/rusty-app"]
