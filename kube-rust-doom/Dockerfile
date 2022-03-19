# Rust build cache STEP
FROM rust:latest as rustbuild-cache

WORKDIR /src
COPY kube-rust-doom-build-cache/src/* src/
COPY kube-rust-doom-build-cache/Cargo.toml .
COPY kube-rust-doom/Cargo.lock .

RUN cargo build --release

# Rust build STEP
FROM rust:latest as rustbuild

WORKDIR /src
COPY --from=rustbuild-cache /src/target/* /src/target/
COPY kube-rust-doom/src/* src/
COPY kube-rust-doom/Cargo.toml .
COPY kube-rust-doom/Cargo.lock .

RUN cargo build --release
RUN cargo install --root /app --path .

# Final bundle 
FROM ubuntu:20.04 
WORKDIR /app

COPY --from=rustbuild /app/bin/kube-rust-doom .

ENTRYPOINT ["/app/kube-rust-doom"]
