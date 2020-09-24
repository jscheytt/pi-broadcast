#!/bin/sh

VERSION=${1:?'VERSION variable missing.'}

sed -i "/^\[package\]$/,/^\[/ s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
