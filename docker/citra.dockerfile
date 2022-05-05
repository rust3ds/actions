FROM dorowu/ubuntu-desktop-lxde-vnc

RUN apt-get update -y && \
    apt-get install -y \
        libqt5gui5 \
        libqt5multimedia5


ARG CITRA_RELEASE=nightly-1763
ARG CITRA_RELEASE_FILE=citra-linux-20220503-856b3d6.tar.xz

WORKDIR /tmp
RUN wget https://github.com/citra-emu/citra-nightly/releases/download/${CITRA_RELEASE}/${CITRA_RELEASE_FILE}
RUN mkdir -p citra && \
    tar --strip-components 1 -C citra -xvf ${CITRA_RELEASE_FILE} && \
    cp citra/citra-qt citra/citra /usr/local/bin

COPY citra/qt-config.ini /root/.config/citra-emu/

ENV OPENBOX_ARGS='--startup "citra-qt /root/hello-world.3dsx"'
