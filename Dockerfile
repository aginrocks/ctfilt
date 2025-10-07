# fuck it, i need it to work asap
# TODO: remove this shitty dockerfile

FROM rust:bookworm

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    clang \
    lld \
    perl \
    cmake \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . .

RUN cargo build --release --bin server

ENTRYPOINT ["/app/target/release/server"]
