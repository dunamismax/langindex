# syntax=docker/dockerfile:1

FROM rust:1.95-slim AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY README.md CONTRIBUTING.md ./
COPY crates ./crates
COPY public ./public
RUN cargo build -p langindex-site --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates curl \
  && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/release/langindex-site /app/langindex-site
COPY src/content /app/src/content
ENV LANGINDEX_SITE_ADDR=0.0.0.0:3000
ENV LANGINDEX_CONTENT_DIR=/app/src/content
EXPOSE 3000
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -fsS http://127.0.0.1:3000/healthz || exit 1
CMD ["/app/langindex-site"]
