FROM rust:latest AS builder

LABEL version="1.0.0"
LABEL description="A simple Rust API to decrypt encrypted embed sources"

ARG PORT=3000
ENV PORT=${PORT}

WORKDIR /app

# Copy Rust project files and build
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release && \
    cp ./target/release/embed_decrypt /app/embed_decrypt

# Copy TypeScript files and install dependencies
COPY package.json package-lock.json tsconfig.json rabbit.ts ./

RUN apt-get update && apt-get install -y curl && \
    curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g ts-node && \
    npm install

# Final Stage: Runtime Environment
FROM debian:bookworm-slim AS final

RUN adduser --disabled-password --gecos "" --uid 10001 appuser && \
    apt-get update && apt-get install -y curl && \
    curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g ts-node

# Copy Rust binary and TypeScript dependencies
COPY --from=builder /app/embed_decrypt /app/embed_decrypt
COPY --from=builder /app/rabbit.ts /app/rabbit.ts
COPY --from=builder /app/tsconfig.json /app/tsconfig.json
COPY --from=builder /app/package.json /app/package.json
COPY --from=builder /app/package-lock.json /app/package-lock.json
COPY --from=builder /app/node_modules /app/node_modules

WORKDIR /app

ENV RUST_LOG=INFO
EXPOSE ${PORT}

ENTRYPOINT ["./embed_decrypt"]
