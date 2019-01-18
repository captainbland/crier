FROM rust:1.31-slim

EXPOSE 9080

WORKDIR /usr/src/myapp
COPY . .

RUN apt-get update && apt-get install -y libssl-dev pkg-config libpq-dev
RUN rustup default nightly
RUN cargo install --path .
RUN cargo install diesel_cli --no-default-features --features "postgres"

CMD ["crier"]