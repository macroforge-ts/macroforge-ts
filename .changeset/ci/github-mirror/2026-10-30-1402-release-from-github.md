# ci(github): release from GitHub and mirror releases between the forges

- **Date:** 2026-10-30 14:02
- **Commit:** a599fe64
- **Area:** CI, GitHub Actions, GitLab releases, website deploy

## Problem

Releases could only be published from GitLab's pipeline, on the self-hosted runner. Releases are API
objects, so the push mirror copied tags but never releases, and GitHub had none. The website deploys
from GitLab's release webhook, so a release made anywhere else would not deploy it.

## Cause

There was no GitHub release workflow and nothing that copied releases between the forges.

## Fix

`release.yml` releases a `vX.Y.Z` tag pushed to GitHub: it checks the tag, verifies through
`ci.yml`, publishes the crates through crates.io trusted publishing, builds the packages, publishes
them to npm and JSR through the job's OIDC token, and creates the GitHub Release. It skips a tag
GitLab already has, since that came through the mirror. `sync-releases.yml` runs every 30 minutes
and after `release.yml`, creating on each forge the releases the other has: a GitLab release is
copied to GitHub, and a GitHub release reaches GitLab by pushing its tag with `ci.skip` and creating
the release as a project service account, which fires the webhook that deploys the website. A tag
naming different commits on the two forges fails the sync instead of being copied. Both forges take
their notes from `tooling/ci/release-notes.md`, and the image setup is a shared action.

## Verified

`actionlint` and `shellcheck` pass, and `deno fmt --check` passes on every changed file. The
workflows have not run on GitHub yet.
