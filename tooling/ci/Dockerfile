# The CI job image. Playwright's Ubuntu image carries Chromium and its system
# libraries; the tag must match the @playwright/test version the playground
# tests resolve. Everything else comes from the repo's pixi.toml at job time.
FROM mcr.microsoft.com/playwright:v1.63.0-noble

# build-essential: the cc that rustc links through (gcc 12+ for -fuse-ld=mold).
# jq: the release job reads the version from tooling/versions.json.
RUN apt-get update \
    && apt-get install -y --no-install-recommends build-essential ca-certificates curl git jq \
    && rm -rf /var/lib/apt/lists/*

# The binary moves out of ~/.pixi: jobs mount a cache volume there for the
# global rustup install, which would hide it.
RUN curl -fsSL https://pixi.sh/install.sh | bash \
    && mv /root/.pixi/bin/pixi /usr/local/bin/pixi
ENV PATH="/root/.pixi/bin:/root/.cargo/bin:${PATH}"

# The digest of this file at build time. Jobs compare it with the checkout's
# copy, so a pipeline never runs on an image built from an older Dockerfile.
ARG DOCKERFILE_SHA256
ENV DOCKERFILE_SHA256=${DOCKERFILE_SHA256}
