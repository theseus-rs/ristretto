#!/usr/bin/env bash
set -euo pipefail

run_apt() {
    local attempt

    for attempt in 1 2 3; do
        if timeout 5m sudo env DEBIAN_FRONTEND=noninteractive apt-get \
            -o Acquire::Retries=3 \
            -o Acquire::http::Timeout=30 \
            -o Acquire::https::Timeout=30 \
            "$@"; then
            return 0
        fi

        echo "apt-get attempt ${attempt} failed; retrying..." >&2
        sleep $((attempt * 5))
    done

    return 1
}

run_apt update
run_apt install -y build-essential libasound2-dev pkg-config
