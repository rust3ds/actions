FROM buildpack-deps:latest as builder

ARG CITRA_CHANNEL=nightly
ARG CITRA_RELEASE=1962

WORKDIR /tmp
COPY ./docker/download_citra.sh /usr/local/bin/download_citra
RUN apt-get update -y && apt-get install -y jq
RUN download_citra ${CITRA_CHANNEL} ${CITRA_RELEASE}

RUN wget https://apt.devkitpro.org/install-devkitpro-pacman && \
    chmod +x ./install-devkitpro-pacman && \
    yes | /tmp/install-devkitpro-pacman
RUN dkp-pacman -S --noconfirm \
        devkitARM-gdb \
        libctru

FROM ubuntu:latest

RUN apt-get update -y && \
    apt-get install -y \
        libswscale5 \
        libsdl2-2.0-0 \
        libavformat58 \
        libavfilter7 \
        xvfb

COPY --from=builder /opt/devkitpro /opt/devkitpro
ENV PATH=/opt/devkitpro/devkitARM/bin:${PATH}

COPY --from=builder /tmp/citra.AppImage /usr/local/bin/citra
COPY ./docker/sdl2-config.ini /root/.config/citra-emu/
COPY ./docker/test-runner.gdb /app/
COPY ./docker/entrypoint.sh /app/

WORKDIR /app

ENTRYPOINT [ "/app/entrypoint.sh" ]
