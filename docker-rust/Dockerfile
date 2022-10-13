FROM rust:1.64 as builder

RUN cargo new --bin docker-rust

WORKDIR ./docker-rust

COPY ./Cargo.toml ./Cargo.toml

COPY ./src ./src

RUN cargo build --release

FROM centos:latest
COPY --from=builder /docker-rust/target/release/docker-rust ./docker-rust

CMD ["/docker-rust"]

