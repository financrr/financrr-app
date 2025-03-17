FROM rust:1.85.0-bookworm

ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies
RUN apt-get update -y && apt-get upgrade -y && apt-get install -y \
    curl \
    bash \
    build-essential \
    libssl-dev \
    lldb \
    rust-lldb \
    gdb \
    g++-multilib \
    lib32stdc++6 \
    libncurses5-dev

ARG UID=1000
ARG GUID=1000

# Create non-root user
RUN groupadd --gid ${GUID} financrr \
  && useradd --uid ${UID} --gid ${GUID} --shell /bin/bash --create-home financrr \
  && mkdir -p /home/financrr/.cargo \
  && chown -R financrr:financrr /home/financrr/.cargo

# Set the working directory
WORKDIR /home/financrr/app

COPY --chown=financrr:financrr rust-toolchain.toml .

# Switch to non-root user
USER financrr

# Create advisory db directory
RUN mkdir -p /home/financrr/.cargo/advisory-db \
  && chown -R financrr:financrr /home/financrr/.cargo/advisory-db

# Install Rust toolchain and components based on rust-toolchain.toml
# This ensures rustup won't need to download components at runtime
RUN rustup show && rustup component list --installed

# Install binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install cargo clis
#RUN --mount=type=cache,target=/home/financrr/.cargo/bin \
#    cargo binstall cargo-nextest@0.9.92 loco@0.14.0 sea-orm-cli@1.1.7 cargo-audit@0.21.2 --secure

CMD ["/usr/bin/env", "bash"]
