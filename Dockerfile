FROM rust:1.31 AS builder
WORKDIR /rpb/src

COPY . .
RUN cargo build --manifest-path=server/Cargo.toml --release




FROM debian
WORKDIR /rpb/bin

COPY --from=builder /rpb/src/server/target/release/server .

EXPOSE 50051
CMD /rpb/bin/server
