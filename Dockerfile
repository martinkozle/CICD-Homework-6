FROM rust:1

WORKDIR /usr/src/mersenne-primes
COPY . .

RUN cargo install --path .

CMD ["mersenne-primes"]
