FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# Install LLVM tools and the lld linker to improve Rust build times.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends clang lld \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*