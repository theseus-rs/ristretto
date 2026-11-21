#!/bin/sh
set -eu
fixture_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
fixture_output=$(mktemp -d)
trap 'rm -rf "$fixture_output"' EXIT HUP INT TERM
javac --release 17 -g -d "$fixture_output" "$fixture_dir/CanonicalStaticFields.java" "$fixture_dir/CanonicalStaticFieldsOracle.java"
cmp "$fixture_output/CanonicalStaticFields.class" "$fixture_dir/CanonicalStaticFields.class"
cmp "$fixture_output/CanonicalStaticFields\$Node.class" "$fixture_dir/CanonicalStaticFields\$Node.class"
java -Xverify:all -cp "$fixture_output" CanonicalStaticFieldsOracle > "$fixture_output/oracle.txt"
cmp "$fixture_output/oracle.txt" "$fixture_dir/static-fields-oracle.txt"
