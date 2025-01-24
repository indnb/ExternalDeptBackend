# Stage 1: Build the application
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /app

# Copy only the dependency files to leverage Docker caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main file to fetch dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

# Copy the rest of the application source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create a lightweight runtime image
FROM debian:bookworm-slim

# Install only the required libraries for the Rust application
RUN apt-get update && apt-get install -y \
    libssl-dev libsqlite3-dev libpq-dev ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/ExternalDeptBackend .

# Optionally copy the .env file if needed
COPY .env /app/.env

# Expose the application's port
EXPOSE 8181

# Set the environment variable for Rust backtraces
ENV RUST_BACKTRACE=1

# Define the default command to run the application
CMD ["./ExternalDeptBackend"]

