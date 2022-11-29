FROM buildpack-deps:latest as builder

# ARG CITRA_RELEASE=nightly-1783
# ARG CITRA_RELEASE_FILE=citra-linux-20220902-746609f.tar.xz

ARG CITRA_CHANNEL=nightly
ARG CITRA_RELEASE=1816

WORKDIR /tmp
COPY ./citra/download_citra.sh /usr/local/bin/download_citra
RUN apt-get update -y && apt-get install -y jq
RUN download_citra ${CITRA_CHANNEL} ${CITRA_RELEASE}

FROM ubuntu:latest

RUN apt-get update -y && \
    apt-get install -y \
        libswscale5 \
        libsdl2-2.0-0 \
        libavformat58 \
        libavfilter7 \
        xvfb

COPY --from=builder /tmp/citra /usr/local/bin
COPY ./citra/sdl2-config.ini /root/.config/citra-emu/

WORKDIR /app

CMD [ "citra", "--version" ]
