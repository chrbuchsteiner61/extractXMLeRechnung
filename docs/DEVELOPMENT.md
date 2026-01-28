# Development Guide

## Project Structure

```
extractXMLeRechnung/
├── src/                    # Source code
│   ├── main.rs            # Application entry point
│   ├── handlers.rs        # HTTP request handlers
│   ├── erechnung_pdf_service.rs         # Business logic
│   ├── models.rs          # Data models
│   ├── pdf.rs             # PDF processing utilities
│   └── errors.rs          # Error definitions
├── tests/                 # Integration tests
├── examples/              # Usage examples
├── docs/                  # Documentation
├── .github/workflows/     # CI/CD pipelines
├── Cargo.toml            # Project dependencies
└── README.md             # Project overview
```

## Development Setup

1. **Install Rust toolchain:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install additional tools:**
   ```bash
   cargo install cargo-watch    # For auto-recompilation
   cargo install cargo-audit     # For security auditing
   ```

3. **Run in development mode:**
   ```bash
   cargo watch -x run
   ```

## Testing

```bash
# Run all tests
cargo test

# Run with coverage
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration_tests
```

## Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Security audit
cargo audit
```

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## Docker Development

Build and run with Docker:

```bash
# Build image
docker build -t extract-xml-rechnung .

# Run container
docker run -p 8080:8080 extract-xml-rechnung
```

Or use docker-compose for development:

```bash
# Start development environment
docker-compose up --build

# Run in background
docker-compose up -d
```

## API Testing

Test the API endpoints:

```bash
# Health check
curl http://localhost:8080/health

# Extract XML from PDF (example)
curl -X POST http://localhost:8080/extract \
  -H "Content-Type: multipart/form-data" \
  -F "file=@example.pdf"
```\n