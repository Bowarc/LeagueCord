###########
## BUILD ##
###########
FROM rust:1.85 AS builder

# Install build dependencies
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked wasm-bindgen-cli

# Setup
WORKDIR /app

# Get build scripts
RUN mkdir ./scripts
COPY ./scripts/build.sh ./scripts/build_back.sh ./scripts/build_front.sh ./scripts

# Copy project
COPY Cargo.toml Cargo.lock Rocket.toml .

COPY ./back ./back
COPY ./front ./front
COPY ./shared ./shared

# Build the whole project
RUN sh ./scripts/build.sh release

#########
## RUN ##
#########
FROM ubuntu:22.04 AS runner
# FROM scratch Causes issues with musl libc ? something like that
# check this for more info https://dev.to/mattdark/rust-docker-image-optimization-with-multi-stage-builds-4b6c
 
WORKDIR /app

COPY --from=builder /app/target/release/leaguecord .
COPY --from=builder /app/Rocket.toml .
COPY ./static ./static
COPY --from=builder /app/target/wasm-bindgen/release/* ./static/
COPY ./.env .

RUN mkdir log

EXPOSE 42069

CMD ["./leaguecord"]
