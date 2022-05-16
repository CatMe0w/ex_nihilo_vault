FROM rust:1.60.0-slim-bullseye as builder
WORKDIR /opt
RUN apt-get update && apt-get install -y libsqlite3-dev xz-utils && USER=root cargo new --bin ex_nihilo_vault
WORKDIR /opt/ex_nihilo_vault
COPY ./Cargo.lock ./Cargo.toml ./vault.db.xz .
RUN cargo build --release && rm ./src/*.rs && rm ./target/release/deps/ex_nihilo_vault*
ADD ./src ./src
RUN xz -d vault.db.xz && cargo build --release

FROM debian:bullseye-20220509-slim
RUN apt-get update && apt-get install -y libsqlite3-0
WORKDIR /opt/ex_nihilo_vault
COPY --from=builder /opt/ex_nihilo_vault/target/release/ex_nihilo_vault /opt/ex_nihilo_vault/vault.db .
COPY ./Rocket.toml .
EXPOSE 8000
CMD ["/opt/ex_nihilo_vault/ex_nihilo_vault"]