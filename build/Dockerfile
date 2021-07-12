FROM rust:1.53

RUN cargo install cargo-build-dependencies
RUN cd /tmp && USER=root cargo new --bin snake
WORKDIR /tmp/snake
COPY Cargo.toml Cargo.lock ./
RUN cargo build-dependencies --release
COPY src /tmp/snake/src
# target/x86_64-unknown-linux-gnu/release/snake
RUN cargo build --release --target=x86_64-unknown-linux-gnu
