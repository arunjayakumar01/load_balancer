# Use a base image that includes cross-compilers
FROM ubuntu:23.04

WORKDIR /usr/src/myapp

# Update package lists, install cross-compilers, libssl, curl, and make sure 'cc' is properly linked to the x86_64 compiler
RUN apt-get update && apt-get install -y \
    gcc-x86-64-linux-gnu \
    g++-x86-64-linux-gnu \
    libssl-dev \
    curl \
    build-essential

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

# Set environment variables to use the correct compilers
ENV CC=x86_64-linux-gnu-gcc
ENV CXX=x86_64-linux-gnu-g++
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc

# Add the target architecture to Rust
RUN rustup target add x86_64-unknown-linux-gnu

# Copy your source code
#COPY . .

# Command to run the application
#CMD ["/usr/src/myapp/target/x86_64-unknown-linux-gnu/release/load_balancer"]
#CMD ["tail", "-f", "/dev/null"]

