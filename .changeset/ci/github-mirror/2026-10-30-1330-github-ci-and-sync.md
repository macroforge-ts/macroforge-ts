# ci(github): verify pull requests on GitHub and sync merged main back to GitLab

- **Date:** 2026-10-30 13:30
- **Commit:** f7bd3db5
- **Area:** CI, GitHub Actions, GitLab mirroring, pixi tasks

## Problem

CI ran only on the self-hosted GitLab runner, so nothing could be verified off that machine. GitHub
received `main` from GitLab's push mirror only, which overwrote any divergent ref, so a merge made
on GitHub would have been erased by the next mirror push.

## Cause

The repository had no GitHub workflows, and the GitLab push mirror was configured as a one-way copy.

## Fix

`.github/workflows/ci.yml` runs `pixi run verify --check` for pull requests into `main` on a
GitHub-hosted runner, in an image built from `tooling/ci/Dockerfile`, with the build and the
container's home on the runner's scratch disk and the home cached between runs.
`.github/workflows/sync-gitlab.yml` pushes `main` to GitLab after every push to GitHub's `main`
through a write deploy key, skipping when GitLab already has the commit and failing without forcing
when the two have diverged. The GitLab mirror keeps divergent refs, so neither side overwrites the
other. The bootstrap both CIs and the docs repeated is now `pixi run setup`.

## Verified

`actionlint` passes on both workflows and `deno fmt --check` on every changed file. `pixi run setup`
ran setup-rust, setup:wasm-bindgen and build:mf to completion locally. The workflows themselves have
not run on GitHub yet.
