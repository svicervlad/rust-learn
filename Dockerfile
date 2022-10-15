FROM rust:1-bullseye as builder
WORKDIR /usr/src/app

RUN rustup target add wasm32-unknown-unknown && cargo install trunk && cargo install wasm-bindgen-cli

COPY . .

RUN trunk build
RUN cargo build --release

FROM rust:1-bullseye
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/backend /usr/src/app/backend
COPY --from=builder /usr/src/app/dist /usr/src/app/dist
ENV ROCKET_LOG_LEVEL=normal
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
EXPOSE 8000
CMD ["./backend"]
