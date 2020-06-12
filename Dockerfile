FROM clux/muslrust as build-env

RUN apt-get update -y && \
    apt-get install -y cmake musl* upx

ADD . /opt
WORKDIR /opt

RUN ln -s "/usr/bin/g++" "/usr/bin/musl-g++"
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN upx /opt/target/x86_64-unknown-linux-musl/release/bid-tracker-rs

FROM gcr.io/distroless/base
COPY --from=build-env /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=build-env /opt/target/x86_64-unknown-linux-musl/release/bid-tracker-rs /
ENTRYPOINT ["/bid-tracker-rs"]