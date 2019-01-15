FROM rust:1.31-slim

WORKDIR /usr/src/myapp
COPY . .

RUN apt-get update && apt-get install -y libssl-dev pkg-config libpq-dev
RUN rustup default nightly
RUN cargo install --path .

CMD ["crier"]