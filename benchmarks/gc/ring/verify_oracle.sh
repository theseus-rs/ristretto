#!/usr/bin/env bash
# Integer model shared by the fixture check and the HotSpot driver.

ring_checksum() {
    local workload=$1 steps=$2 retained base mask
    case $workload in
        small)  retained=8;   base=64;   mask=63 ;;
        medium) retained=32;  base=1024; mask=255 ;;
        large)  retained=128; base=4096; mask=1023 ;;
        *) printf 'Unknown workload: %s\n' "$workload" >&2; return 1 ;;
    esac
    if (( steps < 0 )); then
        printf '%s\n' -1
        return
    fi

    local -a ring
    local i slot=1 generation length first last checksum=1
    for ((i = 0; i < retained; i++)); do
        ring[i]=$i
    done
    for ((i = 0; i < steps + retained; i++)); do
        if (( i < steps )); then
            # Replacement keeps the same slot. The next removal is two slots on.
            generation=${ring[slot]}
            ring[slot]=$((i + retained))
            slot=$(((slot + 2) % retained))
        else
            # The head stays in slot zero, including when its node is replaced.
            generation=${ring[i - steps]}
        fi
        length=$((base + (generation & mask)))
        first=$(((generation + 128) % 256 - 128))
        last=$(((generation * 17 + 3 + 128) % 256 - 128))
        checksum=$(((31 * (checksum + generation) + length + 7 * first + last) & 0xffffffff))
    done
    if (( checksum >= 0x80000000 )); then
        checksum=$((checksum - 0x100000000))
    fi
    printf '%d\n' "$checksum"
}

expected_rows() {
    local workload steps checksum
    for workload in small medium large; do
        for steps in -1 0 1 7 17 65 257; do
            checksum=$(ring_checksum "$workload" "$steps") || return 1
            printf '%s\t%d\t%d\n' "$workload" "$steps" "$checksum"
        done
    done
}

if [[ ${BASH_SOURCE[0]} == "$0" ]]; then
    set -euo pipefail
    if [[ ${1:-} == --help || ${1:-} == -h ]]; then
        printf 'Usage: %s ORACLE.tsv\n' "$0"
        exit 0
    fi
    if (( $# != 1 )); then
        printf 'Usage: %s ORACLE.tsv\n' "$0" >&2
        exit 2
    fi
    awk '!/^#/ { sub(/\r$/, ""); print }' < "$1" | diff -u <(expected_rows) -
    printf '%s\n' '21 HotSpot cases match independent ring model; no performance qualification'
fi
