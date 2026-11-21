#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
javac --release 17 -g -d "$tmp" "$here/CanonicalInitializerFaults.java" "$here/CanonicalInitializerFaultOracle.java"
java -Xverify:all -cp "$tmp" CanonicalInitializerFaultOracle > "$tmp/oracle.txt"
for class in "$here"/*.class; do cmp "$class" "$tmp/$(basename "$class")"; done
cmp "$here/oracle.txt" "$tmp/oracle.txt"
