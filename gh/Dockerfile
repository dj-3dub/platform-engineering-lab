FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    curl \
    git \
    ca-certificates \
    openssh-client \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Install GitHub CLI (gh)
RUN wget https://github.com/cli/cli/releases/download/v2.62.0/gh_2.62.0_linux_amd64.deb \
    && dpkg -i gh_2.62.0_linux_amd64.deb \
    && rm gh_2.62.0_linux_amd64.deb

WORKDIR /workspace

ENTRYPOINT ["tail", "-f", "/dev/null"]
