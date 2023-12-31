# syntax=docker/dockerfile:1
ARG RUST_BUILD_IMG=rust:1.67-slim-bullseye
ARG DEBIAN_TAG=bullseye-slim

FROM $RUST_BUILD_IMG as base

# AMD64
FROM --platform=$BUILDPLATFORM base as builder-amd64
ARG TARGET="x86_64-unknown-linux-gnu"

# ARM64
FROM --platform=$BUILDPLATFORM base as builder-arm64
ARG TARGET="aarch64-unknown-linux-gnu"

FROM builder-$TARGETARCH as builder

RUN adduser --disabled-password --disabled-login --gecos "" --no-create-home efected-coto-emmory
RUN apt update && apt install -y g++ build-essential protobuf-compiler
RUN rustup target add $TARGET

RUN cargo init efected-coto-emmory

WORKDIR /efected-coto-emmory

# touch lib.rs as we combine both
Run touch src/lib.rs

# touch benches as it's part of Cargo.toml
RUN mkdir benches
RUN touch benches/a_benchmark.rs

# copy cargo.*
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml

# cache depencies
RUN mkdir .cargo
RUN cargo vendor > .cargo/config
RUN --mount=type=cache,target=$CARGO_HOME/registry \
    --mount=type=cache,target=$CARGO_HOME/.git \
    --mount=type=cache,target=efected-coto-emmory/target,sharing=locked \
    cargo build --target $TARGET --bin efected-coto-emmory-app --release

COPY src ./src
# copy src
COPY src ./src
# copy benches
COPY benches ./benches

# copy config
COPY config ./config

# final build for release
RUN rm ./target/$TARGET/release/deps/*efected_coto_emmory*
RUN cargo build --target $TARGET --bin efected-coto-emmory-app --release

RUN strip ./target/$TARGET/release/efected-coto-emmory-app

RUN mv ./target/$TARGET/release/efected-coto-emmory* /usr/local/bin
RUN mv ./config /etc/config

FROM debian:${DEBIAN_TAG}

ARG backtrace=0
ARG log_level=info

ENV RUST_BACKTRACE=${backtrace} \
    RUST_LOG=${log_level}

COPY --from=builder /usr/local/bin/efected-coto-emmory* .
COPY --from=builder /etc/config ./config
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

USER efected-coto-emmory:efected-coto-emmory

EXPOSE 3000
EXPOSE 4000
ENTRYPOINT ["./efected-coto-emmory-app"]
