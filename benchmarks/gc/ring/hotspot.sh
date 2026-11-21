#!/usr/bin/env bash
# Capture a finite, paired HotSpot ring comparison in a bounded Linux cgroup.
set -euo pipefail

here=$(cd -- "$(dirname -- "$0")" && pwd)
fixtures="$here/../../../ristretto_vm/tests/fixtures/canonical"
# shellcheck source=verify_oracle.sh
source "$here/verify_oracle.sh"

die() { printf '%s\n' "$*" >&2; exit 1; }

usage() {
    cat <<'EOF'
Usage: hotspot.sh OUTPUT [--workload small|medium|large] [--steps N]
                         [--warmup N] [--samples N] [--forks N]
Defaults: small workload, 257 steps, 10 warmup calls, 8 samples, 3 forks.
EOF
}

bounded_integer() {
    local value=$1 minimum=$2 maximum=$3
    if [[ $value =~ ^\+?0*([0-9]{1,6})$ ]]; then
        value=$((10#${BASH_REMATCH[1]}))
        if (( minimum <= value && value <= maximum )); then
            printf '%d\n' "$value"
            return
        fi
    fi
    die "Expected an integer from $minimum to $maximum, got '$1'"
}

# Expand the kernel's list of CPU numbers and ranges into a sorted JSON array.
allowed_cpus() {
    awk '/^Cpus_allowed_list:/ {
        count = split($2, ranges, ",")
        for (i = 1; i <= count; i++) {
            n = split(ranges[i], ends, "-")
            for (cpu = ends[1] + 0; cpu <= ends[n] + 0; cpu++) print cpu
        }
    }' /proc/self/status | jq -sc '.'
}

resource_limits() {
    local root=/sys/fs/cgroup group directory value quota period
    local -a memory_limits=() cpu_limits=()
    group=$(awk '/^0::/ { print substr($0, 4); exit }' /proc/self/cgroup)
    [[ $group == /* ]] || die 'This benchmark requires a cgroup v2 hierarchy'
    directory=$root${group%/}
    while :; do
        if [[ -r $directory/memory.max ]]; then
            read -r value < "$directory/memory.max"
            [[ $value == max ]] || memory_limits+=("$value")
        fi
        if [[ -r $directory/cpu.max ]]; then
            read -r quota period < "$directory/cpu.max"
            [[ $quota == max ]] || cpu_limits+=("$quota/$period")
        fi
        [[ $directory != "$root" ]] || break
        directory=${directory%/*}
    done
    (( ${#memory_limits[@]} && ${#cpu_limits[@]} )) ||
        die 'Run inside a cgroup with finite memory.max and cpu.max (see README)'
    jq -cn --arg group "$group" --arg memory "${memory_limits[*]}" \
        --arg cpu "${cpu_limits[*]}" --argjson affinity "$(allowed_cpus)" '
        {cgroup: $group, affinity: $affinity,
         memory_max_bytes: ($memory | split(" ") | map(tonumber) | min),
         cpu_quota: ($cpu | split(" ") | map(split("/") | (.[0]|tonumber) / (.[1]|tonumber)) | min)}'
}

run_timed() {
    # Keep interpreter startup outside the measured process lifetime.
    perl -MTime::HiRes=clock_gettime,CLOCK_MONOTONIC -e '
        my $path = shift;
        open my $timing, ">", $path or die "$path: $!";
        my $start = clock_gettime(CLOCK_MONOTONIC);
        system(@ARGV) == 0 or die "HotSpot process failed (status $?)\n";
        printf {$timing} "%.0f\n", (clock_gettime(CLOCK_MONOTONIC) - $start) * 1e9;
    ' -- "$@"
}

output='' workload=small steps=257 warmup=10 samples=8 forks=3
while (( $# )); do
    case $1 in
        -h|--help) usage; exit 0 ;;
        --*=*) set -- "${1%%=*}" "${1#*=}" "${@:2}" ;;
        --workload|--steps|--warmup|--samples|--forks)
            (( $# >= 2 )) || die "Missing value for $1"
            printf -v "${1#--}" '%s' "$2"
            shift 2 ;;
        --) shift; (( $# == 1 )) && [[ -z $output ]] || die 'Expected one output path'
            output=$1; shift ;;
        -*) die "Unknown option: $1" ;;
        *) [[ -z $output ]] || die 'Expected one output path'
            output=$1; shift ;;
    esac
done
[[ -n $output ]] || { usage >&2; exit 2; }
[[ ! -e $output && ! -L $output ]] || die "Output already exists: $output"
case $workload in small|medium|large) ;; *) die "Unknown workload: $workload" ;; esac
steps=$(bounded_integer "$steps" 0 100000)
warmup=$(bounded_integer "$warmup" 0 128)
samples=$(bounded_integer "$samples" 1 128)
forks=$(bounded_integer "$forks" 1 32)

[[ -r /proc/self/cgroup ]] || die 'Run the driver on Linux in a bounded cgroup v2 (see README)'
unset JAVA_TOOL_OPTIONS JDK_JAVA_OPTIONS _JAVA_OPTIONS JDK_JAVAC_OPTIONS
java=$(type -P java) || die 'Put the HotSpot java launcher on PATH'
javac=$(type -P javac) || die 'Put the HotSpot javac compiler on PATH'
affinity=$(allowed_cpus | jq -r '.[:2] | join(",")')
taskset -pc "$affinity" "$$" >/dev/null
limits=$(resource_limits)
processors=$(jq '.affinity | length' <<< "$limits")
checksum=$(ring_checksum "$workload" "$steps")
version=$("$java" -version 2>&1)

tmp=$(mktemp -d)
report=''
trap 'rm -rf -- "$tmp"; if [[ -n $report ]]; then rm -f -- "$report"; fi' EXIT
report=$(mktemp -- "$(dirname -- "$output")/.ring-baseline.XXXXXXXX")
"$javac" --release 17 -g -d "$tmp" "$fixtures/CanonicalRingWorkload.java" "$here/RingSamples.java"
hashes=$(cd -- "$tmp" && sha256sum CanonicalRingWorkload*.class |
    jq -Rn '[inputs | split("  ") | {key: .[1], value: .[0]}] | from_entries')

for ((fork = 0; fork < forks; fork++)); do
    collectors=(G1 ZGC)
    if (( fork % 2 )); then collectors=(ZGC G1); fi
    for collector in "${collectors[@]}"; do
        [[ $(resource_limits) == "$limits" ]] || die 'Resource limits changed during capture'
        if [[ $collector == G1 ]]; then gc_flag=-XX:+UseG1GC; else gc_flag=-XX:+UseZGC; fi
        flags=(-Xms64m -Xmx64m "-XX:ActiveProcessorCount=$processors" "$gc_flag")
        run_timed "$tmp/process.ns" timeout --signal=KILL 300 "$java" "${flags[@]}" -cp "$tmp" RingSamples \
            "$workload" "$steps" "$checksum" "$warmup" "$samples" > "$tmp/run.tsv"

        jq -Rnc --argjson fork "$fork" --arg collector "$collector" \
            --argjson flags "$(printf '%s\n' "${flags[@]}" | jq -Rn '[inputs]')" \
            --argjson elapsed "$(< "$tmp/process.ns")" --argjson checksum "$checksum" \
            --argjson count "$samples" '
            [inputs | split("\t")] as $rows
            | [$rows[] | select(.[0] == "collector" and length == 2) | .[1]] as $beans
            | [$rows[] | select(.[0] == "sample" and length == 3)
               | {checksum: (.[1]|tonumber), elapsed_ns: (.[2]|tonumber)}] as $samples
            | (if $collector == "G1" then "G1" else "Z" end) as $prefix
            | if ($rows|length) == (($beans|length) + $count)
                 and any($beans[]; startswith($prefix)) and ($samples|length) == $count
                 and all($samples[]; .checksum == $checksum and .elapsed_ns >= 0)
              then {fork: $fork, collector: $collector, beans: $beans, flags: $flags,
                    process_ns: $elapsed, samples: $samples}
              else error("unexpected collector or sample data") end' \
            "$tmp/run.tsv" >> "$tmp/processes.jsonl"
    done
done

jq -s --arg version "$version" --arg java "$(realpath -- "$java")" \
    --argjson limits "$limits" --arg workload "$workload" --argjson steps "$steps" \
    --argjson checksum "$checksum" --argjson warmup "$warmup" --argjson hashes "$hashes" '
    {schema: 1, jdk: $version, java: $java, limits: $limits, workload: $workload,
     steps: $steps, expected: $checksum, warmup: $warmup, class_sha256: $hashes,
     processes: .}' "$tmp/processes.jsonl" > "$report"
ln -T -- "$report" "$output"
