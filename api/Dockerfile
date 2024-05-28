FROM rust:latest
WORKDIR /usr/src/app
RUN cargo build api --release
EXPOSE 8000
CMD ["./target/release/api"]
