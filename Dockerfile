FROM rust:latest
WORKDIR /usr/src/app
COPY . .
WORKDIR /usr/src/app/client
RUN trunk build
WORKDIR /usr/src/app/client
RUN cargo build --bin server --release
EXPOSE 8000
CMD ["./target/release/server"]
