FROM rust:latest

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

# Build and cache the dependencies
# RUN cargo build --release

# Copy the source code to the container
COPY src ./src

RUN cargo install sccache

# Build the Rust application
# RUN cargo build --release

# Expose any ports that the application needs to listen on
EXPOSE 8000

# Run the Rust application
CMD ["./target/release/my_rust_app"]
