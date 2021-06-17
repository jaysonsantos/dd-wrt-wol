FROM rust:1.53-slim-buster as builder

RUN apt update \
    && apt install -y build-essential cmake 

COPY . /code
WORKDIR /code
RUN cargo build --release \
    && mv target/release/dd-wrt-wol-api /

FROM debian:buster-slim
COPY --from=builder /dd-wrt-wol-api /dd-wrt-wol-api
CMD [ "sh", "-exc", "exec /dd-wrt-wol-api $(echo \"$HOSTS_CONFIG\" | tr ';' '\n' | while read i; do echo \"-h$i\"; done)" ]
