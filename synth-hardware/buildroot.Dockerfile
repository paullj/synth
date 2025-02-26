FROM ubuntu:22.04

# Buildroot version to use
ARG BUILDROOT_VERSION=2024.05.2
ARG BUILDROOT_TARGET=raspberrypizero2w_defconfig

# Install wget to download buildroot
RUN apt-get update && apt install -y wget
RUN wget https://buildroot.org/downloads/buildroot-${BUILDROOT_VERSION}.tar.gz
RUN tar xvzf buildroot-${BUILDROOT_VERSION}.tar.gz

# Install dependencies
RUN apt-get update && apt install -y patch cpio python3 unzip rsync bc bzip2 ncurses-dev git make g++ file

WORKDIR /buildroot-${BUILDROOT_VERSION}

RUN make ${BUILDROOT_TARGET}

# Start a shell for debugging
CMD ["make", "menuconfig"]
