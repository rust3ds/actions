FROM ghcr.io/ian-h-chamberlain/rust-devkitarm

RUN apt-get update -y && \
    apt-get install -y python3 python3-pip python3-pil

RUN pip3 install vncdotool

COPY driver/vncdo.sh /usr/libexec/

CMD [ "vncdo", "--version" ]
