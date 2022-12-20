FROM rust:1.61.0 AS builder

WORKDIR /licenses
COPY . .

RUN cargo install --path .

CMD ["licenses", "--help"]
