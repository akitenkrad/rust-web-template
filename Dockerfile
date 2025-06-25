FROM rust:slim-bookworm AS builder
WORKDIR /app

ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

# Install Rust and Cargo
RUN apt update -y && \
    apt install -y curl build-essential libssl-dev poppler-utils libopencv-dev clang libclang-dev && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release


FROM rust:slim-bookworm as runtime
WORKDIR /app

# Install Rust and Cargo
RUN apt update -y && \
    apt install -y curl build-essential libssl-dev poppler-utils libopencv-dev clang libclang-dev && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

# Install dependencies
RUN cargo install cargo-make && \
    cargo install cargo-nextest

RUN adduser app && usermod -aG sudo app && chown -R app /app
COPY --from=builder /app/target/release/app ./target/release/app