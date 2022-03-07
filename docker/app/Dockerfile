FROM rust:1.59.0-slim-bullseye

# Copy code
COPY ./Cargo.* ./
COPY ./src/ ./src/

# Compile
RUN cargo build --release

ENTRYPOINT ["target/release/rusty-app"]
