FROM rust:slim-bullseye as builder

RUN apt-get update -qq

WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust:slim-bullseye as runner
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
COPY --from=builder /app/target/release/catalog-service /usr/local/bin/catalog-service
COPY --from=builder /app/Rocket.toml /usr/local/bin/Rocket.toml

WORKDIR /usr/local/bin
CMD ["catalog-service"]
