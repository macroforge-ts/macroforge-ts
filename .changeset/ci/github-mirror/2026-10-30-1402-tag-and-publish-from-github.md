# feat(tooling): tag and publish a release from GitHub

- **Date:** 2026-10-30 14:02
- **Commit:** c76ed3f3
- **Area:** mf tag, mf publish-local

## Problem

`mf tag` could only push a release tag to the remote named `origin`, so a release could not start on
GitHub. On GitHub Actions, `publish-local --non-interactive` failed npm's `npm whoami` check and
demanded a `JSR_TOKEN`, though npm and `deno publish` authenticate there through the job's OIDC
token.

## Cause

The tag command and its git helpers hardcoded `origin`, and publish-local recognised trusted
publishing for npm only through GitLab's `NPM_ID_TOKEN` and for JSR not at all.

## Fix

`mf tag --forge github|gitlab` (GitLab by default) finds the one remote whose URL is on that forge's
host, and fetches, checks and pushes the tag there; a retag deletes GitHub's unprotected tag with
`git push --delete` and GitLab's protected one through its API. publish-local treats a GitHub
Actions job allowed to request an OIDC token as trusted publishing for npm, and accepts it in place
of `JSR_TOKEN`.

## Verified

`cargo test -p mf`: 140 and 10 passed, including five new tests of the URL host parsing. Clippy with
`-D warnings` is clean on all of mf's targets. `mf tag --dry-run` resolved `origin` and
`mf tag --forge github --dry-run` resolved `github` against this checkout's remotes, then stopped at
the main-branch check.
