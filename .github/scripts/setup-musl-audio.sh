#!/usr/bin/env sh
set -eu

# Alpine does not package a static libasound, but Rust's musl target links static binaries.
alsa_version="1.2.14"
alsa_archive="alsa-lib-${alsa_version}.tar.bz2"
alsa_sha512="2716cc3a2299da4a1a170d734af082d78dc452b253179d0f1a9ec190140734aecf002b6924eec4ff2699ce88ce1ae5c56821c267f36384910984db726d1f9626"
alsa_build_dir="$(mktemp -d)"
trap 'rm -rf "${alsa_build_dir}"' EXIT

apk add --no-cache build-base curl linux-headers pkgconf
cd "${alsa_build_dir}"
curl --proto '=https' --tlsv1.2 --fail --silent --show-error --location --remote-name \
  "https://www.alsa-project.org/files/pub/lib/${alsa_archive}"
printf '%s  %s\n' "${alsa_sha512}" "${alsa_archive}" | sha512sum -c -
tar -xjf "${alsa_archive}"
cd "alsa-lib-${alsa_version}"
./configure \
  --prefix=/usr \
  --enable-static \
  --disable-shared \
  --disable-python \
  --disable-resmgr \
  --enable-rawmidi \
  --enable-seq \
  --enable-aload \
  --disable-dependency-tracking \
  --without-versioned
make -j"$(getconf _NPROCESSORS_ONLN)"
make install
