# Base pre-built cross image
ARG CROSS_BASE_IMAGE
FROM $CROSS_BASE_IMAGE

# Add our foreign architecture and install our dependencies
RUN apt-get update && apt-get install -y --no-install-recommends apt-utils
RUN dpkg --add-architecture armhf
RUN apt-get update && apt-get -y install libasound2-dev:armhf

# Add our linker search paths and link arguments
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_RUSTFLAGS="-L /usr/lib/arm-linux-gnueabihf -C link-args=-Wl,-rpath-link,/usr/lib/arm-linux-gnueabihf $CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_RUSTFLAGS"
