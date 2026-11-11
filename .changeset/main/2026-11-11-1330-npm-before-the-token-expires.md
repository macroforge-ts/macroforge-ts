# fix(release): publish every npm package before the job's ID token can expire

- **Date:** 2026-11-11 13:30
- **Commit:** pending
- **Area:** `mf publish-local`, the GitLab `publish-packages` job

## Problem

Every GitLab release failed `publish-packages` twice before a third attempt finished: v0.4.0 and
v0.5.0 each did. Each failed attempt stopped on `npm error code ENEEDAUTH` at the first npm publish
past five minutes into the job, between 5m09s and 6m18s, after the earlier packages had published.

## Cause

npm exchanges the job's `NPM_ID_TOKEN` for a publish token on every `npm publish`, and GitLab minted
that ID token with five minutes of lifetime, whatever the job's `timeout`. GitLab sets an ID token's
lifetime from the job's timeout when it mints the token and falls back to five minutes when the
timeout is not yet recorded, which it is only once the job starts running. `mf publish-local`
published each package to npm, then waited up to two minutes for npm to serve it before publishing
it to JSR, so the npm publishes ran well past five minutes. A failed exchange is logged by npm only
at verbose level, and the publish goes on without credentials, so the job showed `ENEEDAUTH`. A
retry got a new token and skipped what npm had, so the third attempt had little enough left.

## Introduced

`be5369875` (2026-09-27, "GitLab CI: verify merge requests, publish tagged releases") moved the npm
publishes to trusted publishing on a GitLab-hosted runner with `NPM_ID_TOKEN`, keeping the wait for
npm between one package's publishes and the next's. The `timeout: 1h` and its comment, added after
v0.4.0 on the reading that the token lasts the job's timeout, did not change the lifetime: v0.5.0
failed at the same point.

## Fix

`mf publish-local` publishes every pending npm package back to back, then every pending JSR package,
each JSR publish waiting until npm serves its package and the ones before it. Before each npm
publish it reads `NPM_ID_TOKEN`'s `exp` claim and fails, naming the token's remaining lifetime, when
fewer than 15 seconds are left. The `publish-packages` comment says what the timeout does not
guarantee.

## Verified

`cargo test -p mf --bin mf publish_local`: 5 passed, the new tests that the lifetime is read from
`exp`, that an expired token has none left, and that a value that is not a JWT, a payload that is
not base64url and a payload without `exp` are refused. Clippy on `mf`, with all targets and features
and warnings denied, is clean. A dry run of the release's npm and JSR publish for 0.6.0,
`mf publish-local --dry-run --skip-build --registry npm --registry jsr`, published all seven npm
packages, steps 1 to 7, before the eight JSR packages, steps 8 to 15. No release has run with it:
only a tag pipeline publishes.
