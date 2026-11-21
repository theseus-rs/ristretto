#!/bin/sh
set -eu
fixture_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
fixture_output=$(mktemp -d)
trap 'rm -rf "$fixture_output"' EXIT HUP INT TERM
javac --release 17 -g -d "$fixture_output" "$fixture_dir/CanonicalClassInitialization.java" "$fixture_dir/CanonicalEntryInitialization.java" "$fixture_dir/CanonicalInitializationOracle.java"
for compiled in "$fixture_output"/CanonicalClassInitialization*.class "$fixture_output"/CanonicalEntryInitialization.class; do
    cmp "$compiled" "$fixture_dir/$(basename -- "$compiled")"
done
java -Xverify:all -cp "$fixture_output" CanonicalInitializationOracle > "$fixture_output/oracle.txt"
cmp "$fixture_output/oracle.txt" "$fixture_dir/initialization-oracle.txt"
