# ─── Stage 1: Builder ───────────────────────────────────────────────────────
FROM rust:1.83-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app

# Cache dependencies separately (improves rebuild times)
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --locked 2>&1 | tail -5 || true
RUN rm -f target/release/deps/smiled_server*

# Build the actual application
COPY src ./src
RUN cargo build --release --locked

# ─── Stage 2: Runtime ───────────────────────────────────────────────────────
FROM alpine:3.21 AS runtime

# Install runtime dependencies
RUN apk add --no-cache ca-certificates curl libgcc

# Create non-root user
RUN addgroup -g 1001 smiled && adduser -u 1001 -G smiled -s /bin/sh -D smiled

WORKDIR /app

# Copy compiled binary
COPY --from=builder /app/target/release/smiled-server /app/smiled-server

# Set ownership
RUN chown -R smiled:smiled /app

USER smiled

EXPOSE 8080

# Health check
HEALTHCHECK --interval=10s --timeout=5s --start-period=10s --retries=5 \
  CMD curl -f http://localhost:8080/api/v1/health || exit 1

CMD ["/app/smiled-server"]
