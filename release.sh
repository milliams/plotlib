#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

[[ -n ${1+x} ]] || (echo "Usage: release.sh <version number>" && exit 1)

version=$1
date=$(date --iso-8601)

echo -e "\\n# Checking version number\\n"
git fetch
git tag --list | grep -q "${version}" && echo "Error: Git tag ${version} already exists" && exit 1

echo -e "\\n# Updating version number in files\\n"
sed --in-place "s/version = \".*\"/version = \"${version}\"/" Cargo.toml
sed --in-place "s/## Unreleased/## Unreleased\\n\\n## ${version} - ${date}/" CHANGELOG

echo -e "\\n# Adding to Git\\n"
git add Cargo.toml CHANGELOG
git commit -m "Update version numbers for ${version}"

echo -e "\\n# Pushing to Git\\n"
git tag "${version}"
git push origin
git push origin --tags

echo -e "\\n# Success ✓"
