FROM rust:1.86.0-alpine3.21 AS builder

RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    curl

RUN apk update

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM alpine:3.22.1

ENV PORT=8080
ENV UID=1000
ENV GUID=1000
ENV LOCO_ENV=production

# Add curl and update package repositories
RUN apk add --no-cache curl
RUN apk update

# Create a non-root user
RUN addgroup -g ${UID} -S financrr && \
    adduser -u ${GUID} -S -D -G financrr -H -h /home/financrr -s /bin/sh financrr

# Set the working directory
WORKDIR /home/financrr

# Creating config directory
RUN mkdir -p /home/financrr/config

# Copy the binaries from the builder stage
COPY --from=builder --chown=${UID}:${GUID} /usr/src/config/docker.yaml /home/financrr/config/production.yaml
COPY --from=builder --chown=${UID}:${GUID} /usr/src/target/release/financrr-cli /home/financrr/financrr-cli

# Set permissions for the binary
RUN chmod +x /home/financrr/financrr-cli

# Set permissions for config directory and all files within it
RUN chown -R ${UID}:${GUID} /home/financrr/config

# Create a directory for logs
RUN mkdir /home/financrr/logs
RUN chown -R ${UID}:${GUID} /home/financrr/logs

# Create data folder
RUN mkdir /home/financrr/data
RUN chown -R ${UID}:${GUID} /home/financrr/data

# Expose the port
EXPOSE ${PORT}

# Switch to non-root user
USER financrr

# Entrypoint command
ENTRYPOINT ["/home/financrr/financrr-cli"]
