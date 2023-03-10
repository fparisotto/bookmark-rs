# builder
FROM rust:1.66.0-bullseye as builder

# new empty project
RUN USER=root cargo new --bin daemon
WORKDIR /daemon

# copy manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./wait-for-it.sh ./wait-for-it.sh

# cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/daemon*
RUN cargo build --release

# runner
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && update-ca-certificates

COPY --from=builder /daemon/target/release/daemon .
COPY --from=builder /daemon/wait-for-it.sh .

RUN chmod +x wait-for-it.sh

CMD ["./daemon"]
