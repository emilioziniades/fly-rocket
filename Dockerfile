# setup cargo chef for optimal layer caching
FROM rust:1.68 AS chef 
RUN cargo install cargo-chef 
WORKDIR app

# cache dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# build project
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin fly-rocket 

# actual application
FROM debian:buster-slim AS runtime
RUN apt update
RUN apt install -y libpq-dev
WORKDIR app
COPY --from=builder /app/target/release/fly-rocket /usr/local/bin
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/fly-rocket"]

