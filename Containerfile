# Build stage
FROM fedora:latest AS builder

EXPOSE 3000

# Install build dependencies
RUN dnf install -y cargo gcc openssl-devel \
    && dnf clean all

WORKDIR /build

# Copy manifest and source
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY html ./html

# Build the release binary
RUN cargo build --release

# Runtime stage
FROM fedora:latest

WORKDIR /app

# Copy the binary from the builder stage
# Renaming brick_1 to snap as requested
COPY --from=builder /build/target/release/brick_1 ./snap

# Copy static assets (required by the application)
COPY html ./html

# Create the data directory and declare a volume for persistence
RUN mkdir data
VOLUME /app/data

# Set the entrypoint
CMD ["./snap"]
