FROM rust:1.53-slim-buster as planner

RUN cargo install cargo-chef

WORKDIR /code
COPY . ./
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.53-slim-buster as cacher
RUN apt update \
    && apt install -y build-essential cmake
WORKDIR /code
COPY --from=planner /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef
COPY --from=planner /code/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.53-slim-buster as builder
RUN apt update \
    && apt install -y build-essential cmake
WORKDIR /code
COPY . ./
COPY --from=cacher /code/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release \
    && mv target/release/dd-wrt-wol-api /

FROM debian:buster-slim
COPY --from=builder /dd-wrt-wol-api /dd-wrt-wol-api
CMD [ "sh", "-exc", "exec /dd-wrt-wol-api $(echo \"$HOSTS_CONFIG\" | tr ';' '\n' | while read i; do echo \"-h$i\"; done)" ]
