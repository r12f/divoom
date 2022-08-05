#
# Dockerfile for running divoom-gateway container.
#
# To create the container and specify the device to control on the runtime, we can use the following command to specify the environment variables:
#
#     docker container create -e DEVICE_ADDRESS=192.168.0.164 <image-name>
#
# To enable debug logs, we can add this additional environment variable in the command line: -e RUST_LOG=debug
#


#
# Global definitions
#

# It is always better to have a shell and support for basic tools, such as wget and etc.
# This only adds ~1.2MB to the final image, which worths every single penny.
FROM busybox

# Environment variables
ENV DEVICE_ADDRESS=127.0.0.1
ENV GATEWAY_ADDRESS=0.0.0.0
ENV GATEWAY_PORT=20821

#
# Build container
#

# Install fonts
RUN mkdir -p /usr/share/fonts/truetype/

COPY *.ttf /
RUN install -m644 *.ttf /usr/share/fonts/truetype/
RUN rm ./*.ttf

# Copy gateway binary
COPY divoom-gateway /

#
# Runtime
#

# Expose port
EXPOSE $GATEWAY_PORT/tcp

# Entrypoint
CMD ["sh", "-c", "./divoom-gateway ${DEVICE_ADDRESS} -s ${GATEWAY_ADDRESS} -p ${GATEWAY_PORT}"]