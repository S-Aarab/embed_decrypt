FROM rust:latest AS builder

LABEL version="1.0.0"
LABEL description="A simple rust API to decrypt encrypted embed sources"

ARG PORT=3000
ENV PORT=${PORT}

WORKDIR /app

COPY . .

RUN \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  cargo build --release && \
  cp ./target/release/embed_decrypt /usr/local/bin/

FROM debian:bookworm-slim AS final

RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "10001" \
  appuser

COPY --from=builder /usr/local/bin/embed_decrypt /usr/local/bin/

WORKDIR /usr/local/bin

ENV RUST_LOG=INFO

EXPOSE ${PORT}

ENTRYPOINT ["embed_decrypt"]

