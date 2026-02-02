# snap

A tiny web service that turns pasted CLI output into an immutable, shareable,
terminal‑style snapshot.

## Build

### Non-Containerized

For local development without containers, ensure you have Rust installed.

Run the application:
```bash
cargo run
```

The server listens on `http://localhost:3000`.

### Containerized

The project includes helper scripts for building and running with Podman or
Docker.

#### 1. Build the Image
```bash
./build.sh
```

#### 2. Run the Container
```bash
./run.sh
```

This will start the container and map port 3000. Data is persisted in a volume
named `snap-data`.

**Configuration:**
To expose the service on a different port (e.g., 8080), use the `-p` argument:
```bash
./run.sh -p 8080
```
