FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# Codespaces expects a writable home directory at /home/codespace.
RUN mkdir -p /home/codespace && chown -R vscode:vscode /home/codespace

# Install LLVM tools and the lld linker to improve Rust build times.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends clang lld \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*