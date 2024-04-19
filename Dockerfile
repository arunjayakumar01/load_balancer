FROM --platform=linux/amd64 alpine:latest

WORKDIR /usr/src/myapp

# Update package lists, install native compilers, openssl (libssl), curl, and build essentials
RUN apk update && apk add --no-cache \
    musl-dev \
    gcc \
    g++ \
    curl \
    make \
    openssl-dev

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

# Add the target architecture to Rust, assuming native compilation
RUN rustup target add x86_64-unknown-linux-musl


