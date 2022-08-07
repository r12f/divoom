#
# Dockerfile for running divoom-gateway container.
#
# To create the container and specify the device to control on the runtime, we can use the following command to specify the environment variables:
#
#     docker container create -e DEVICE_ADDRESS=192.168.0.123 <image-name>
#
# To enable debug logs, we can add this additional environment variable in the command line: -e RUST_LOG=debug
#

#
# Build container
#
FROM --platform=$BUILDPLATFORM busybox AS builder

ARG TARGETPLATFORM
ARG BUILDPLATFORM

# Install fonts
RUN mkdir -p /usr/share/fonts/truetype/

COPY Dockerfile *.ttf /
RUN if [ -f "Amiga 1200.ttf" ]; then install -m644 *.ttf /usr/share/fonts/truetype; fi
RUN if [ -f "Amiga 1200.ttf" ]; then rm ./*.ttf; fi

# Install divoom-gateway binary
RUN case "${TARGETPLATFORM}" in "linux/386") PKG_ARCH="x86"; ;; "linux/amd64") PKG_ARCH="x64"; ;; "linux/arm64") PKG_ARCH="arm64"; ;; *) PKG_ARCH="arm"; ;; esac; wget "http://github.com/r12f/divoom/releases/download/{build.version}/divoom-gateway.{build.version}.linux.${PKG_ARCH}.tar.gz" -O divoom-gateway.tar.gz
RUN tar zxvf divoom-gateway.tar.gz
RUN rm divoom-gateway.tar.gz

#
# Final container
#

# We use alpine to start with, because it is always better to have a shell and support for basic tools, such as wget and etc.
# We have tried to use busybox, but it turns out it somehow kills the network for the server, so we need a "more real" linux to start with.
FROM alpine:3.15.5

# Environment variables
ENV DEVICE_ADDRESS=127.0.0.1
ENV GATEWAY_ADDRESS=0.0.0.0
ENV GATEWAY_PORT=20821
ENV GATEWAY_EXTRA_ARGS=""

# Copy key binaries and resource files
RUN mkdir -p /usr/share/fonts/
COPY --from=builder /usr/share/fonts/truetype /usr/share/fonts/truetype
COPY --from=builder /divoom-gateway /bin

#
# Runtime
#

# Expose port
EXPOSE $GATEWAY_PORT/tcp

# Entrypoint
CMD ["sh", "-c", "/bin/divoom-gateway ${DEVICE_ADDRESS} -s ${GATEWAY_ADDRESS} -p ${GATEWAY_PORT} ${GATEWAY_EXTRA_ARGS}"]
