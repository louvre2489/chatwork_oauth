FROM ekidd/rust-musl-builder:stable AS builder

# Chrome
#RUN sudo apt update && \
#    sudo apt install wget -y && \
#    sudo sh -c 'echo "deb http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list' && \
#    sudo wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | sudo apt-key add - && \
#    sudo apt update && \
#    sudo apt install google-chrome-stable -y
#RUN apk add --update \
#        udev \
#        ttf-freefont \
#        chromium \
#        pkgconfig \
#        libressl-dev \
#        alpine-sdk

ADD --chown=rust:rust . ./

RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add udev \
                       ttf-freefont \
                       chromium

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/chatwork_oauth \
    /usr/local/bin/
# CMD /usr/local/bin/chatwork_oauth
ENTRYPOINT /usr/local/bin/chatwork_oauth

# RUN cargo install --path .
#ENTRYPOINT ["chatwork_oauth"]
