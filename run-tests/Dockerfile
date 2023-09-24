FROM buildpack-deps:latest as builder

WORKDIR /tmp
COPY ./docker/download_citra.sh /usr/local/bin/download_citra
RUN apt-get update -y && apt-get install -y jq

ARG CITRA_CHANNEL=nightly
ARG CITRA_RELEASE=1995
RUN download_citra ${CITRA_CHANNEL} ${CITRA_RELEASE}

FROM devkitpro/devkitarm:latest as devkitarm

# For some reason, citra isn't always happy when you try to run it for the first time,
# so we build a simple dummy program to force it to create its directory structure
RUN cd /opt/devkitpro/examples/3ds/graphics/printing/hello-world && \
    echo 'int main(int, char**) {}' > source/main.c && \
    make && \
    mv hello-world.3dsx /tmp/

FROM ubuntu:latest

RUN --mount=type=cache,sharing=locked,target=/var/cache/apt \
    apt-get update -y && \
    apt-get install -y \
        libswscale5 \
        libsdl2-2.0-0 \
        libavformat58 \
        libavfilter7 \
        xvfb

COPY --from=devkitarm /opt/devkitpro /opt/devkitpro
# There's no way to copy ENV values from other stages properly:
# https://github.com/moby/moby/issues/37345
# Luckily in this case we know exactly what the values should be:
ENV DEVKITPRO=/opt/devkitpro
ENV DEVKITARM=${DEVKITPRO}/devkitARM
ENV PATH=${DEVKITARM}/bin:${PATH}

COPY --from=builder /tmp/citra.AppImage /usr/local/bin/citra
COPY --from=devkitarm /tmp/hello-world.3dsx /tmp/
# We run citra once before copying our config file, so it should create its
# necessary directory structure and run once with defaults
RUN xvfb-run citra --appimage-extract-and-run /tmp/hello-world.3dsx; \
    rm -f /tmp/hello-world.3dsx
# Initial run seems to miss this one directory so just make it manually
RUN mkdir -p /root/.local/share/citra-emu/log

COPY ./docker/sdl2-config.ini /root/.config/citra-emu/
COPY ./docker/test-runner.gdb /app/
COPY ./docker/entrypoint.sh /app/

WORKDIR /app

ENTRYPOINT [ "/app/entrypoint.sh" ]
