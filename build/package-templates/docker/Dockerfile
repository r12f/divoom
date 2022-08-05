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
FROM busybox AS builder

# Install fonts
RUN mkdir -p /usr/share/fonts/truetype/

COPY *.ttf /
RUN if [ -f *.ttf ]; then install -m644 *.ttf /usr/share/fonts/truetype; fi
RUN if [ -f *.ttf ]; then rm ./*.ttf; fi

# Install divoom-gateway binary
RUN wget http://github.com/r12f/divoom/releases/download/{build.version}/divoom-gateway.{build.version}.linux.{build.arch}.tar.gz -O divoom-gateway.tar.gz
RUN tar zxvf divoom-gateway.tar.gz
RUN rm divoom-gateway.tar.gz

#
# Runtime
#
# It is always better to have a shell and support for basic tools, such as wget and etc.
# This only adds ~1.2MB to the final image, which worths every single penny.
FROM busybox

# Environment variables
ENV DEVICE_ADDRESS=127.0.0.1
ENV GATEWAY_ADDRESS=0.0.0.0
ENV GATEWAY_PORT=20821

# Copy key binaries and resource files
COPY --from=builder /*.ttf /
COPY --from=builder /divoom-gateway /bin

# Expose port
EXPOSE $GATEWAY_PORT/tcp

# Entrypoint
CMD ["sh", "-c", "/bin/divoom-gateway ${DEVICE_ADDRESS} -s ${GATEWAY_ADDRESS} -p ${GATEWAY_PORT}"]
