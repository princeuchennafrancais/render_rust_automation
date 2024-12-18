# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set the working directory in the container
WORKDIR /app

# Copy the Cargo.toml and the source code
COPY Cargo.toml Cargo.toml
COPY src/ src/

# Build the project (this will download dependencies)
RUN cargo build --release

# Create a new smaller image for running the application
FROM debian:bullseye-slim

# Install required libraries for the Rust application
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder image
COPY --from=builder /app/target/release/render_rust_automation .

# Set the command to run the application
CMD ["./render_rust_automation"]
