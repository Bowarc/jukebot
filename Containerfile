##########
#  BASE  #
##########
FROM rust:1.85 AS base

# RUN rustup target add wasm32-unknown-unknown
# RUN cargo install --locked wasm-bindgen-cli
# RUN cargo install sccache
RUN cargo install cargo-chef
RUN apt-get update
RUN apt-get install -y cmake

##########
# PANNER #
##########
FROM base AS planner

WORKDIR /app

# Move the essentials
COPY ./Cargo.toml ./Cargo.lock .
COPY ./src ./src

# Prepare all dependencies
RUN cargo chef prepare --recipe-path recipe.json

###########
# BUILDER #
###########
FROM base AS builder

WORKDIR /app

# Take the recipe only from tyhe planner
COPY --from=planner /app/recipe.json recipe.json

# Set up the project's build artefacts
RUN cargo chef cook --release --recipe-path recipe.json

# Pull the projects code
COPY ./scripts/build.sh ./scripts/
COPY ./Cargo.toml ./Cargo.lock .
COPY ./src ./src

# Build it
RUN sh ./scripts/build.sh release

##########
# RUNNER #
##########
FROM ubuntu:22.04 AS runner

WORKDIR /app

RUN apt-get update
RUN apt-get install python3 -y # Python is needed to run ytdlp anyway
RUN apt-get install python3-pip -y
RUN python3 -m pip install -U "yt-dlp[default]"

COPY --from=builder /app/target/release/jukebot .
COPY ./scripts/init.sh ./scripts/
COPY ./.env ./.env

RUN sh ./scripts/init.sh

CMD ["./jukebot"]
