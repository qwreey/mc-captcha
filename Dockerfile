FROM rust:alpine3.21 AS base

RUN apk upgrade --no-cache && apk add musl-dev libressl-dev
COPY . /app
RUN \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/root/.cargo/registry \
    cd /app && \
    cargo build --release && \
    cp /app/target/release/mc-captcha /
RUN apk del musl-dev libressl-dev && rm -rf /app
ENTRYPOINT [ "/mc-captcha" ]
