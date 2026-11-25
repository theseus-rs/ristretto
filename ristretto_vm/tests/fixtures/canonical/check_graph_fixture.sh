#!/bin/sh
# Regenerate the embedded Java 17 fixture in a temporary directory and run it.
set -eu
fixture_dir=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
classes=$(mktemp -d)
trap 'rm -rf -- "$classes"' EXIT
javac --release 17 -g -d "$classes" "$fixture_dir/CanonicalGraphWorkload.java" "$fixture_dir/CanonicalGraphOracle.java"
for class in CanonicalGraphWorkload 'CanonicalGraphWorkload$Node'; do
    cmp "$classes/$class.class" "$fixture_dir/$class.class"
done
java -cp "$classes" CanonicalGraphOracle
