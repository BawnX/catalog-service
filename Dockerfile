FROM rust:slim-bullseye as builder

RUN apt-get update -qq

WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust:slim-bullseye as runner
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
COPY --from=builder /app/target/release/catalog-service /usr/local/bin/catalog-service

WORKDIR /usr/local/bin
CMD ["catalog-service"]
