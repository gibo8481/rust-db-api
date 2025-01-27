FROM rust:1.75 as builder

# Install sqlx-cli for migrations
RUN cargo install sqlx-cli --no-default-features --features postgres

WORKDIR /usr/src/app
COPY . .

# Build the application
RUN cargo build --release

FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copy the binary and other necessary files
COPY --from=builder /usr/src/app/target/release/rust-db-api .
COPY --from=builder /usr/src/app/.env .
COPY --from=builder /usr/src/app/migrations ./migrations
COPY --from=builder /usr/cargo/bin/sqlx /usr/local/bin/sqlx

# Create an entrypoint script
RUN echo '#!/bin/bash\n\
echo "Running database migrations..."\n\
sqlx migrate run\n\
echo "Starting the application..."\n\
./rust-db-api' > /usr/local/bin/entrypoint.sh && \
    chmod +x /usr/local/bin/entrypoint.sh

EXPOSE 8080
CMD ["/usr/local/bin/entrypoint.sh"]
