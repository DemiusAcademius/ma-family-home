# Stage 1: Build the application
FROM rust:latest AS builder

# Set the working directory
WORKDIR /usr/src/my-family-home

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Install system dependencies for SQLx
RUN apt-get update && \
    apt-get install -y libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Build the application
RUN cargo build --release

# Stage 2: Create the runtime image
FROM gcr.io/distroless/cc-debian12

# Copy the binary from the builder stage
COPY --from=builder /usr/src/my-family-home/target/release/my-family-home /app/my-family-home

# Set the working directory
WORKDIR /app

# Expose the port the app runs on
EXPOSE 8080

# Run the binary
CMD ["/app/my-family-home"]
