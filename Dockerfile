FROM namely/protoc-all@sha256:33e47b2aece23a282a9f8d03a193c063cdd4b1f60d427b148b3c449b51a3ba3c

# Install Python pip.
RUN set -ex && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
    python3-pip

# Install Python dev dependencies.
COPY ./dev-requirements.txt /tmp/
RUN python3 -m pip install --upgrade pip && \
    python3 -m pip install --requirement /tmp/dev-requirements.txt

# Install Rust cargo.
RUN set -ex && \
    apt-get install -y --no-install-recommends \
        curl \
        build-essential

# Switch user
ARG uid=1000
RUN useradd -u ${uid} -s /bin/sh -m builder

USER builder
WORKDIR /home/builder

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/home/builder/.cargo/bin:${PATH}"
