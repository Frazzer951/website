# Build Stage
FROM rust:latest as build

# Install dependencies and add targets
RUN apt-get update && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl wasm32-unknown-unknown && \
    cargo install trunk wasm-bindgen-cli && \
    apt-get autoremove -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy source code and build
WORKDIR /usr/src/website
COPY . .

RUN cd frontend && \
    trunk build --release
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime Stage
FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/website/target/x86_64-unknown-linux-musl/release/backend /usr/local/bin/backend
COPY --from=build /usr/src/website/frontend/dist /usr/local/bin/dist

WORKDIR /usr/local/bin
EXPOSE 80
CMD [ "backend" ]
