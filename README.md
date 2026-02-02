# snap

A tiny web service that turns pasted CLI output into an immutable, shareable,
terminal‑style snapshot.

## Features

- **Simple UI:** Minimalist interface for pasting and viewing content.
- **Immutable Snaps:** Once created, snapshots are persisted and assigned a
  unique ID.
- **Terminal Styling:** Optimized for displaying CLI output exactly as it
  appeared.
- **Container Ready:** Built-in support for Podman and Docker.

## Prerequisites

### Local Installation

To build and run `snap` locally, you need the Rust toolchain installed.

1. **Install Rust:**
   The recommended way is via [rustup](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Follow the on-screen instructions to complete the installation.

2. **C Compiler:**
   Ensure you have a C compiler installed (e.g., `gcc` or `clang`), as the
   SQLite dependency may require it during compilation.

## Build & Run

### Non-Containerized

Run the application directly using Cargo:

```bash
cargo run
```

The server listens on `http://localhost:3000`. Data is persisted in a local
`data/` directory.

### Containerized

The project includes helper scripts for building and running with Podman or
Docker.

1. **Build the Image:**
   ```bash
   ./build.sh
   ```

2. **Run the Container:**
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

## Development

We follow a specific development model using "bricks" and "invariants". For more
information on how to contribute or understand the project structure, please
refer to [AGENTS.md](./AGENTS.md).

## License

This project is licensed under the [MIT License](./LICENSE).
