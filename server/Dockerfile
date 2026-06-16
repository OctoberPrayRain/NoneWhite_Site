FROM rust:1.87-slim-bookworm AS builder
WORKDIR /app
COPY server/Cargo.toml server/Cargo.lock ./
COPY server/src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update   && apt-get install -y --no-install-recommends ca-certificates   && rm -rf /var/lib/apt/lists/*   && useradd --create-home --uid 10001 appuser
COPY --from=builder /app/target/release/nonewhite-site-server /usr/local/bin/nonewhite-site-server
RUN mkdir -p /app/uploads && chown -R appuser:appuser /app
USER appuser
ENV HOST=0.0.0.0     PORT=3000     RUST_LOG=info     UPLOAD_DIR=/app/uploads     UPLOAD_PUBLIC_BASE_URL=/uploads
EXPOSE 3000
CMD ["nonewhite-site-server"]
