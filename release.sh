#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

[[ -n ${1+x} ]] || (echo "Usage: release.sh <version number>" && exit 1)

version=$1
date=$(date --iso-8601)

echo -e "\\n# Checking version number\\n"
git fetch
git tag --list | grep -q "${version}" && echo "Error: Git tag ${version} already exists" && exit 1

echo -e "\\n# Checking GitHub credentials\\n"
[[ -n ${GITHUB_TOKEN+x} ]] || (echo "Error: GITHUB_TOKEN variable not set" && exit 1)
curl --fail --silent --output /dev/null -H "Authorization: token ${GITHUB_TOKEN}" https://api.github.com/user

echo -e "\\n# Checking crates.io credentials\\n"
[[ -f ~/.cargo/credentials ]] || (echo "Error: crates.io credentials not present" && exit 1)
cargo publish --dry-run --quiet

echo -e "\\n# Updating version number in files\\n"
sed --in-place "s/version = \".*\"/version = \"${version}\"/" Cargo.toml
sed --in-place "s/## Unreleased/## Unreleased\\n\\n## ${version} - ${date}/" CHANGELOG

echo -e "\\n# Adding to Git\\n"
git add Cargo.toml CHANGELOG
git commit -m "Update version numbers for ${version}"

echo -e "\\n# Publishing crate\\n"
cargo publish

echo -e "\\n# Pushing to Git\\n"
git tag "${version}"
git push origin --tags

echo -e "\\n# Publishing GitHub release\\n"
changes=$(sed '/^## '"${version}"'.*/,/^## .*/!d;//d' CHANGELOG)
changes=$(echo "${changes}" | awk '{printf "%s\\n", $0}')
curl -H "Authorization: token ${GITHUB_TOKEN}" -d '{"tag_name": "'"${version}"'", "name": "'"${version}"'", "body": "'"${changes}"'"}' https://api.github.com/repos/milliams/plotlib/releases | jq

echo -e "\\n# Success ✓"
