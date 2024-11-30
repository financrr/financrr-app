FROM rust:1.82.0-alpine3.20 as builder

RUN apk add --no-cache \
    musl-dev \
    openssl-dev

RUN apk update

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM alpine:3.20.3

ENV PORT=8080
ENV UID=1000
ENV GUID=1000

# Add curl and update package repositories
RUN apk add --no-cache curl
RUN apk update

# Update package repositories
RUN apk add --no-cache curl
RUN apk update

# Create a non-root user
RUN addgroup -g ${UID} -S financrr && \
    adduser -u ${GUID} -S -D -G financrr -H -h /home/financrr -s /bin/sh financrr

# Set the working directory
WORKDIR /home/financrr

# Creating config directory
RUN mkdir -p /home/financrr/config
RUN chown -R ${UID}:${GUID} /home/financrr/config

# Copy the binaries from the builder stage
COPY --from=builder /usr/src/config/docker.yaml /home/financrr/config/production.yaml
COPY --from=builder /usr/src/target/release/financrr-cli /home/financrr/financrr-cli

# Set permissions for the binary
RUN chmod +x /home/financrr/financrr-cli

# Create a directory for logs
RUN mkdir "./logs"
RUN chown -R ${UID}:${GUID} "./logs"

# Expose the port
EXPOSE ${PORT}

# Switch to non-root user
USER financrr

# Entrypoint command
ENTRYPOINT ["/home/financrr/financrr-cli"]

# TODO move cmds below to compose file when deployment docs are ready
CMD ["start", "--server-and-worker"]
CMD ["scheduler", "--tag", "all"]
