FROM rust:trixie
WORKDIR /
RUN cargo new keykit
WORKDIR /keykit
RUN cargo add crossterm
RUN cargo build
COPY main.rs src/main.rs
CMD cargo run
