FROM buildpack-deps:latest as builder

WORKDIR /tmp
COPY ./docker/download_citra.sh /usr/local/bin/download_citra
RUN apt-get update -y && apt-get install -y jq

ARG CITRA_CHANNEL=nightly
ARG CITRA_RELEASE=1995
RUN download_citra ${CITRA_CHANNEL} ${CITRA_RELEASE}

FROM ubuntu:latest

RUN --mount=type=cache,sharing=locked,target=/var/cache/apt \
    apt-get update -y && \
    apt-get install -y \
        libswscale5 \
        libsdl2-2.0-0 \
        libavformat58 \
        libavfilter7 \
        xvfb

COPY --from=devkitpro/devkitarm:latest /opt/devkitpro /opt/devkitpro
# There's no way to copy ENV values from other stages properly:
# https://github.com/moby/moby/issues/37345
# Luckily in this case we know exactly what the values should be:
ENV DEVKITPRO=/opt/devkitpro
ENV DEVKITARM=${DEVKITPRO}/devkitARM
ENV PATH=${DEVKITARM}/bin:${PATH}

COPY --from=builder /tmp/citra.AppImage /usr/local/bin/citra
COPY ./docker/sdl2-config.ini /app/
COPY ./docker/test-runner.gdb /app/
COPY ./docker/entrypoint.sh /app/

WORKDIR /app

ENTRYPOINT [ "/app/entrypoint.sh" ]
