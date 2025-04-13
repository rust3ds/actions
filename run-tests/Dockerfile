FROM buildpack-deps:latest AS builder

WORKDIR /tmp
COPY ./docker/download_emulator.sh /usr/local/bin/download_emulator
RUN apt-get update -y && apt-get install -y jq

FROM devkitpro/devkitarm:latest AS devkitarm

# For some reason, the emulator isn't always happy when you try to run it for the first time,
# so we build a simple dummy program to force it to create its directory structure
RUN cd /opt/devkitpro/examples/3ds/graphics/printing/hello-world && \
    echo 'int main(int, char**) {}' > source/main.c && \
    make && \
    mv hello-world.3dsx /tmp/

FROM ubuntu:noble AS with-nix

# https://git.lix.systems/lix-project/lix-installer#in-a-container
RUN apt-get update -y && apt-get install -y curl

RUN curl --proto '=https' --tlsv1.2 -sSf -L https://install.lix.systems/lix | sh -s -- install linux \
  --extra-conf "sandbox = false" \
  --extra-conf "experimental-features = flakes nix-command" \
  --init none \
  --no-confirm

ENV PATH="$PATH:/nix/var/nix/profiles/default/bin"

# Azahar version2120.2
ARG NIXPKGS_COMMIT=6a39c6e495eefabc935d8ddf66aa45d85b85fa3f
ARG FLAKE=github:NixOS/nixpkgs/${NIXPKGS_COMMIT}
RUN nix flake prefetch "${FLAKE}"
RUN bash -c 'nix profile install "${FLAKE}"#{azahar,xdg-utils,xvfb-run}'

COPY --from=devkitarm /opt/devkitpro /opt/devkitpro
# There's no way to copy ENV values from other stages properly:
# https://github.com/moby/moby/issues/37345
# Luckily in this case we know exactly what the values should be:
ENV DEVKITPRO=/opt/devkitpro
ENV DEVKITARM=${DEVKITPRO}/devkitARM
ENV PATH=${DEVKITARM}/bin:${PATH}

COPY ./docker/sdl2-config.ini /root/.config/azahar-emu/sdl2-config.ini
COPY --from=devkitarm /tmp/hello-world.3dsx /tmp/

RUN export XDG_RUNTIME_DIR="/run/user/0"; \
    xvfb-run azahar -n /tmp/hello-world.3dsx && \
    rm -f /tmp/hello-world.3dsx
# Initial run seems to miss this one directory so just make it manually
RUN mkdir -p /root/.local/share/azahar-emu/log

COPY ./docker/test-runner.gdb /app/
COPY ./docker/entrypoint.sh /app/

WORKDIR /app

ENTRYPOINT [ "/app/entrypoint.sh" ]
