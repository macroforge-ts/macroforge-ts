#!/bin/sh
# Runs a shell command in the macroforge-ci image on a GitHub-hosted runner,
# with the build and the home on the scratch disk the ci-image action prepared.
# The checkout belongs to the runner user, so git, running as root inside, needs
# it marked safe. CARGO_REGISTRY_TOKEN passes through when the step sets it.
set -eu

command=${1:?usage: run-in-image.sh <command>}
: "${CI_HOME:?run the ci-image action first}"
: "${CI_TARGET:?run the ci-image action first}"

exec docker run --rm \
    --volume "$GITHUB_WORKSPACE:/work" \
    --volume "$CI_TARGET:/work/target" \
    --volume "$CI_HOME:/root" \
    --workdir /work \
    --env GIT_CONFIG_COUNT=1 \
    --env GIT_CONFIG_KEY_0=safe.directory \
    --env GIT_CONFIG_VALUE_0=/work \
    --env CARGO_REGISTRY_TOKEN \
    macroforge-ci \
    bash -c "$command"
