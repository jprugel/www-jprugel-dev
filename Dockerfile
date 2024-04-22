FROM rust:latest
WORKDIR /usr/src/app
COPY . .
RUN cargo build --bin server --release
EXPOSE 8000
CMD ["./target/release/server"]
