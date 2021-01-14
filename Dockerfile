FROM ekidd/rust-musl-builder:stable AS builder

ADD --chown=rust:rust . ./

RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add udev \
                       ttf-freefont \
                       chromium

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/chatwork_oauth \
    /usr/local/bin/

ENTRYPOINT /usr/local/bin/chatwork_oauth
