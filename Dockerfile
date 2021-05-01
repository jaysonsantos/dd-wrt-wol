FROM rust:1.51 as builder
RUN apt update \
    && apt install -y musl-tools \
    && rustup target add x86_64-unknown-linux-musl

COPY . /code
WORKDIR /code
RUN cargo build --release --target=x86_64-unknown-linux-musl \
    && mv target/x86_64-unknown-linux-musl/release/dd-wrt-wol-api /

FROM busybox:1
COPY --from=builder /dd-wrt-wol-api /dd-wrt-wol-api
CMD [ "sh", "-exc", "exec /dd-wrt-wol-api $(echo \"$HOSTS_CONFIG\" | tr ';' '\n' | while read i; do echo \"-h$i\"; done)" ]
