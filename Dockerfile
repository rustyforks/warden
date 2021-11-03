### Set up caching via cargo-chef
FROM lukemathwalker/cargo-chef:latest-rust-1.56.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
# Create a lock file for the application
RUN cargo chef prepare --recipe-path recipe.json

### Builder stage
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build project dependencies and cache them using Docker layer
RUN cargo chef cook --release --recipe-path recipe.json
# Build the application
COPY . .
RUN cargo build --release --bin warden

### Runtime stage
FROM debian:bullseye-slim AS runtime
WORKDIR /app
# Install any required dependencies and then clean up artifacts
RUN apt-get update -y && \
    apt-get install -y --no-install-recommends openssl && \
    apt-get autoremove -y && \
    apt-get clean -y && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/warden warden
ENTRYPOINT ["./warden"]
