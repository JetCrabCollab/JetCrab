# Multi-stage build for JetCrab
FROM rust:1.75 as builder

WORKDIR /app

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

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
COPY --from=builder /app/target/release/claw /usr/local/bin/claw
RUN chmod +x /usr/local/bin/jetcrab /usr/local/bin/claw

# Switch to non-root user
USER jetcrab

# Set working directory
WORKDIR /home/jetcrab

# Expose default port (if needed)
EXPOSE 3000

# Default command
CMD ["jetcrab", "--help"]
