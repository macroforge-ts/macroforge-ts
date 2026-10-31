# ci(github): upload the snapshots a failed run would record

- **Date:** 2026-10-30 18:42
- **Commit:** 073ae84ce, 3db7333ba (fixup)
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

A failed run uploads every `.snap.new` outside `node_modules` as the `pending-snapshots` artifact,
and `CONTRIBUTING.md` says how to download and accept them. The fixup leaves out `node_modules`,
where Deno's links to the crate repeat every snapshot.

## Verified

`actionlint` passes on `ci.yml`. Run 41146044781 uploaded the artifact; it held the five library
snapshots, and their copies under `node_modules` that the fixup excludes.
