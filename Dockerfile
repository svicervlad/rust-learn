FROM rust:1-alpine as builder
WORKDIR /usr/src/app

RUN rustup target add wasm32-unknown-unknown && cargo install trunk && cargo install wasm-bindgen-cli

COPY . .

RUN trunk build
RUN cargo build --release

FROM alpine:latest
WORKDIR /usr/local/bin
COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/backend
COPY --from=builder /usr/src/app/dist /usr/local/dist
CMD ["backend"]
