FROM rust:latest
WORKDIR /usr/src/app
COPY . .
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
WORKDIR /usr/src/app/client
RUN trunk build
WORKDIR /usr/src/app
RUN cargo build --bin server --release
EXPOSE 8000
CMD ["./target/release/server"]
