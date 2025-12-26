FROM rust:trixie
WORKDIR /
RUN apt-get update && apt-get install -y libasound2-dev
RUN cargo new keykit
WORKDIR /keykit
RUN cargo add crossterm
RUN cargo add rodio
RUN cargo build
COPY main.rs src/main.rs
RUN touch src/main.rs
RUN cargo build
CMD cargo run
