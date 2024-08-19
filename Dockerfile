ARG RUST_VERSION=1.80.0
ARG APP_NAME=hirata-store-api

FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release

FROM debian:bullseye-slim AS final

RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/hirata-store-api /usr/local/bin/hirata-store-api
WORKDIR /usr/local/bin

EXPOSE 8000

CMD ["hirata-store-api"]
