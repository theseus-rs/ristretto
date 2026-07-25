#!/usr/bin/env sh

set -eu

target="$1"
case "$target" in
    sparcv9-sun-solaris)
        compiler="sparcv9-sun-solaris2.10-gcc"
        ;;
    x86_64-pc-solaris)
        compiler="x86_64-pc-solaris2.10-gcc"
        ;;
    *)
        echo "Unsupported Solaris target: $target" >&2
        exit 1
        ;;
esac

# The cross-rs Solaris images use a Solaris 10 sysroot, while current Rust
# links these libc functions by their Solaris 11 XPG7 names.
"$compiler" -std=c99 -Wall -Werror -fPIC -x c -c -o /tmp/ristretto-solaris-compat.o - <<'EOF'
#include <sys/socket.h>
#include <unistd.h>

extern int __xnet_socket(int, int, int);

long __sysconf_xpg7(int name) {
    return sysconf(name);
}

int __xnet7_socket(int domain, int type, int protocol) {
    return __xnet_socket(domain, type, protocol);
}
EOF
