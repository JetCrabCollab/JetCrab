# Multi-stage build for JetCrab (build from workspace root: docker build -f JetCrab/Dockerfile .)
FROM rust:1.82-bookworm as builder

WORKDIR /app

COPY . .

RUN cargo build --release -p jetcrab

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -s /bin/false jetcrab

# Copy binaries from builder stage
COPY --from=builder /app/target/release/jetcrab /usr/local/bin/jetcrab
RUN chmod +x /usr/local/bin/jetcrab

# Switch to non-root user
USER jetcrab

# Set working directory
WORKDIR /home/jetcrab

# Expose default port (if needed)
EXPOSE 3000

# Default command
CMD ["jetcrab", "--help"]
