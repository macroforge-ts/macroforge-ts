#!/usr/bin/env bash
# Creates on each forge the releases the other has and it lacks, with the same
# name and notes. A tag GitLab lacks is pushed with ci.skip first, so GitLab
# does not run its release pipeline for a version already published. A tag that
# names different commits on the two forges is an error, never copied.
#
# Runs in GitHub Actions after the gitlab action, with GH_TOKEN and
# GITLAB_RELEASE_TOKEN (a Developer on the GitLab project) set.
set -euo pipefail

: "${GITLAB_REPO:?run the gitlab action first}"
: "${GITLAB_API:?run the gitlab action first}"
: "${GH_TOKEN:?GH_TOKEN is not set}"
if [ -z "${GITLAB_RELEASE_TOKEN:-}" ]; then
    echo "::error::The GITLAB_RELEASE_TOKEN secret is not set."
    exit 1
fi

gitlab_api() {
    local response
    if ! response="$(curl --fail-with-body --silent --show-error \
        --header "PRIVATE-TOKEN: $GITLAB_RELEASE_TOKEN" "$@")"; then
        echo "::error::GitLab API request failed: $response" >&2
        return 1
    fi
    printf '%s\n' "$response"
}

# Every GitLab release, one JSON object per line.
gitlab_releases() {
    local page=1 batch
    while :; do
        batch="$(gitlab_api "$GITLAB_API/releases?per_page=100&page=$page")"
        if [ "$(jq length <<< "$batch")" -eq 0 ]; then
            break
        fi
        jq -c '.[]' <<< "$batch"
        page=$((page + 1))
    done
}

# The commit GitLab's tag names, or nothing when GitLab lacks the tag. An
# annotated tag lists its peeled commit last.
gitlab_tag_commit() {
    local refs
    refs="$(git ls-remote --tags "$GITLAB_REPO" "refs/tags/$1" "refs/tags/$1^{}")"
    awk 'END { print $1 }' <<< "$refs"
}

has_line() {
    grep --quiet --line-regexp --fixed-strings -- "$1" <<< "$2"
}

gitlab_json="$(gitlab_releases)"
gitlab_tags="$(jq -r .tag_name <<< "$gitlab_json")"
github_tags="$(gh api --paginate "repos/$GITHUB_REPOSITORY/releases" --jq '.[] | select(.draft | not) | .tag_name')"
# Copying an older release must not make it GitHub's latest.
newest="$(printf '%s\n%s\n' "$gitlab_tags" "$github_tags" | sort --version-sort | awk 'NF { last = $0 } END { print last }')"
failed=0

while IFS= read -r release; do
    if [ -z "$release" ]; then
        continue
    fi
    tag="$(jq -r .tag_name <<< "$release")"
    if has_line "$tag" "$github_tags"; then
        continue
    fi
    if ! git show-ref --verify --quiet "refs/tags/$tag"; then
        echo "::notice::$tag has not been mirrored here yet; a later run copies its release."
        continue
    fi
    here="$(git rev-parse "refs/tags/$tag^{commit}")"
    there="$(jq -r .commit.id <<< "$release")"
    if [ "$here" != "$there" ]; then
        echo "::error::$tag is $here here but $there on GitLab; move one to match, then rerun."
        failed=1
        continue
    fi

    notes="$(jq -r '.description // ""' <<< "$release")"
    links="$(jq -r '.assets.links[]? | "- [\(.name)](\(.url))"' <<< "$release")"
    if [ -n "$links" ]; then
        notes="$notes"$'\n\n'"$links"
    fi
    latest=false
    if [ "$tag" = "$newest" ]; then
        latest=true
    fi
    gh release create "$tag" --verify-tag --latest="$latest" \
        --title "$(jq -r .name <<< "$release")" --notes "$notes"
    echo "Copied the $tag release from GitLab."
done <<< "$gitlab_json"

while IFS= read -r tag; do
    if [ -z "$tag" ] || has_line "$tag" "$gitlab_tags"; then
        continue
    fi
    here="$(git rev-parse "refs/tags/$tag^{commit}")"
    there="$(gitlab_tag_commit "$tag")"
    if [ -z "$there" ]; then
        git push --push-option=ci.skip "$GITLAB_REPO" "refs/tags/$tag"
        echo "Pushed $tag to GitLab without a pipeline."
    elif [ "$here" != "$there" ]; then
        echo "::error::$tag is $here here but $there on GitLab; move one to match, then rerun."
        failed=1
        continue
    fi

    payload="$(gh release view "$tag" --json name,body |
        jq --arg tag "$tag" '{tag_name: $tag, name: .name, description: .body}')"
    gitlab_api --request POST --header 'Content-Type: application/json' \
        --data "$payload" "$GITLAB_API/releases" > /dev/null
    echo "Copied the $tag release to GitLab."
done <<< "$github_tags"

exit "$failed"
