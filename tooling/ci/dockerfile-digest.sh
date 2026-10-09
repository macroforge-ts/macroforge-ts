#!/bin/sh
# The SHA-256 of what Docker reads from a Dockerfile: its leading parser
# directives and its instructions, without comments or blank lines, so editing
# a comment keeps the digest. The image build records it and every job checks
# it, so both run this script.
set -eu

dockerfile=${1:?usage: dockerfile-digest.sh <Dockerfile>}

# A `#` line inside a heredoc is part of the heredoc, not a comment, so a
# Dockerfile with one cannot be normalized by lines.
if grep -qE '<<-?["'\'']?[A-Za-z_]' "$dockerfile"; then
    echo "dockerfile-digest.sh: $dockerfile uses a heredoc, which this digest cannot normalize" >&2
    exit 1
fi

if command -v sha256sum >/dev/null 2>&1; then
    digest() { sha256sum; }
else
    digest() { shasum -a 256; }
fi

awk '
    BEGIN { header = 1 }
    header && /^[[:space:]]*#[[:space:]]*[A-Za-z]+[[:space:]]*=/ { print; next }
    { header = 0 }
    /^[[:space:]]*#/ { next }
    /^[[:space:]]*$/ { next }
    { print }
' "$dockerfile" | digest | cut -c1-64
