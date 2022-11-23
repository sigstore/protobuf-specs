FROM namely/protoc-all@sha256:07f1ba9dbe11f5675e2efc8617c9552217dc4c3eb5ccd108f7c3889878dbae50

# Install Python pip.
RUN set -ex && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
    python3-pip

# Install Python dev dependencies.
COPY ./dev-requirements.txt /tmp/
RUN python3 -m pip install --upgrade pip && \
    python3 -m pip install --requirement /tmp/dev-requirements.txt
