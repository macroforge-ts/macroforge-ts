# The CI job image. The testground's end-to-end suite drives Chromium's
# headless shell and nothing else, so that is the only browser the image
# carries. Everything else comes from the repo's pixi.toml at job time.
FROM ubuntu:noble

# build-essential: the cc that rustc links through (gcc 12+ for -fuse-ld=mold).
# jq: the release job reads the version from crates/macroforge_ts/package.json.
RUN apt-get update \
    && apt-get install -y --no-install-recommends build-essential ca-certificates curl git jq \
    && rm -rf /var/lib/apt/lists/*

# The binary moves out of ~/.pixi: jobs mount a cache volume there for the
# global rustup install, which would hide it.
RUN curl -fsSL https://pixi.sh/install.sh | bash \
    && mv /root/.pixi/bin/pixi /usr/local/bin/pixi
ENV PATH="/root/.pixi/bin:/root/.cargo/bin:${PATH}"

# Each Playwright release drives one browser revision, so this must match the
# @playwright/test version tooling/testground/tests resolves. Node exists only
# for the install and leaves the image with its caches in the same layer.
ARG PLAYWRIGHT_VERSION=1.63.0
ENV PLAYWRIGHT_BROWSERS_PATH=/ms-playwright
RUN pixi exec --spec nodejs npx --yes "playwright@${PLAYWRIGHT_VERSION}" install --with-deps --only-shell chromium \
    && rm -rf /root/.cache /root/.npm /var/lib/apt/lists/*

# The digest of this file's instructions at build time, from
# tooling/ci/dockerfile-digest.sh. Jobs compare it with the checkout's copy, so
# a pipeline never runs on an image built from older instructions.
ARG DOCKERFILE_SHA256
ENV DOCKERFILE_SHA256=${DOCKERFILE_SHA256}
