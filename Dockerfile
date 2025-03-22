FROM rust:1.85 AS base

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked wasm-bindgen-cli
# RUN cargo install sccache 
RUN cargo install cargo-chef 

FROM base AS planner

WORKDIR /app

COPY ./Rocket.toml ./Cargo.toml ./Cargo.lock .
COPY ./back ./back
COPY ./front ./front
COPY ./shared ./shared

RUN cargo chef prepare --recipe-path recipe.json

FROM base AS builder

WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo chef cook -p front --release --target=wasm32-unknown-unknown --recipe-path recipe.json

COPY ./scripts/build.sh ./scripts/build_back.sh ./scripts/build_front.sh ./scripts/
COPY ./Rocket.toml ./Cargo.toml ./Cargo.lock .
COPY ./back ./back
COPY ./front ./front
COPY ./shared ./shared

RUN sh ./scripts/build.sh release


FROM ubuntu:22.04 AS runner

WORKDIR /app

# Here we take the rocket config from builder because it has been used to build the front end, to elimiate all TOCTOU / desync issues, we use the same one
COPY --from=builder /app/target/release/leaguecord /app/Rocket.toml .
COPY ./static ./static
COPY --from=builder /app/target/wasm-bindgen/release/* ./static/
COPY ./.env .

RUN mkdir ./log

EXPOSE 42069

CMD ["./leaguecord"]

