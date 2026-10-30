# ci(github): upload the snapshots a failed run would record

- **Date:** 2026-10-30 18:42
- **Commit:** 073ae84ce
- **Area:** GitHub CI

## Problem

When a conformance golden moved, a GitHub CI run failed and the `.snap.new` files insta wrote were
lost with the runner, so accepting them meant running the suite again locally.

## Cause

`ci.yml` kept nothing from a failed run.

## Introduced

379620155 (2026-10-30), `ci(github): verify, release and mirror through GitHub alongside GitLab`,
added the workflow without it.

## Fix

A failed run uploads every `.snap.new` as the `pending-snapshots` artifact, and `CONTRIBUTING.md`
says how to download and accept them.

## Verified

`actionlint` passes on `ci.yml`.
