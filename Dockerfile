FROM rust:1.55-alpine as builder
COPY . /code
WORKDIR /code
RUN cargo build --release \
    && mv target/release/dd-wrt-wol-api / \
    && mv target/release/dd-wrt-wol-cli /

FROM alpine
COPY --from=builder /dd-wrt-wol-api /dd-wrt-wol-api
COPY --from=builder /dd-wrt-wol-cli /dd-wrt-wol-cli
CMD [ "sh", "-exc", "exec /dd-wrt-wol-api $(echo \"$HOSTS_CONFIG\" | tr ';' '\n' | while read i; do echo \"-h$i\"; done)" ]
