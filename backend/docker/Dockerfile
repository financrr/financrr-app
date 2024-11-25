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

# Copy the binaries from the builder stage
COPY --from=builder /usr/src/config/docker.yaml /usr/app/config/production.yaml
COPY --from=builder /usr/src/target/release/financrr-cli /home/financrr/financrr-cli

# Set permissions for the binary
RUN chmod +x /home/financrr/financrr_backend-cli

RUN mkdir "./logs"
RUN chown -R ${UID}:${GUID} "./logs"

# Expose the port
EXPOSE ${PORT}

# Switch to non-root user
USER financrr

# Entrypoint command
ENTRYPOINT ["/home/financrr/financrr-cli"]

CMD ["start", "--server-and-worker"]
