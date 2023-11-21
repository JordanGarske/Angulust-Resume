FROM rust:1.71 as build
# Install the nightly toolchain
RUN rustup default nightly-2023-04-01
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
# Set the working directory for the build stage
WORKDIR /app

# Copy the entire content of your Rust application into the container's /app directory
COPY . .
RUN  apt-get install -y openssl
# Change the working directory to /app/rustume/src and build the Rust application using Cargo
WORKDIR /app/rust/rustume/src
RUN cargo build --release
# set up the postgres database so all tool are supported
RUN apt-get update && apt-get install -y libpq-dev
# Stage 2: Create the final minimal image
FROM debian:buster

# Copy the binary from the build stage to the final image
COPY --from=build /app/rust/rustume/target/release/rustume /usr/local/bin/rustume

# Set the working directory in the final image (optional, not necessary for binary execution)
WORKDIR /usr/local/bin
RUN apt-get update && apt-get install -y openssl


CMD ["rustume"]
EXPOSE 8080