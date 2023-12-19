
# Stage 1: Build stage
FROM rust:1.71 as build

# Install the nightly toolchain
RUN rustup default nightly-2023-04-01

# Set the working directory for the build stage
WORKDIR /app

# Copy the entire content of your Rust application into the container's /app directory
COPY . .
# Install dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Change the working directory to /app/rustume/src and build the Rust application using Cargo
WORKDIR /app/rust/rustume/src
RUN cargo build --release

# Stage 2: Create the final minimal image
FROM debian:bullseye-slim as final

# Install necessary libraries
RUN apt-get update && \
    apt-get install -y libpq5 && \
    rm -rf /var/lib/apt/lists/*
RUN apt-get update && apt-get install -y ca-certificates
# Copy the binary from the build stage to the final image
COPY --from=build /app/rust/rustume/target/release/rustume /usr/local/bin/rustume
COPY --from=build /app/rust/rustume/Rocket.toml /usr/local/bin
COPY --from=build /app/rust/rustume/.env /usr/local/bin
COPY /rust/rustume/static /app/rust/rustume/static

# Set the working directory in the final image (optional, not necessary for binary execution)
WORKDIR /usr/local/bin

# Expose the port your Rust application is using
EXPOSE 8080

# Define the default command to run your Rust application
CMD ["rustume"]