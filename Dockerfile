FROM rust:1.52
WORKDIR /usr/src/questrade-scripts
COPY . .

RUN cargo build --release

CMD ["./target/release/questrade-scripts"]
