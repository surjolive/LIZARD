#!/usr/bin/env sh
set -eu

release_base_url="${LIZARD_RELEASE_BASE_URL:-https://github.com/surjolive/LIZARD/releases/latest/download}"
install_dir="${LIZARD_INSTALL_DIR:-$HOME/.local/bin}"
os_name="$(uname -s)"
arch_name="$(uname -m)"

case "$os_name:$arch_name" in
  Linux:x86_64) asset="lizard-linux-x86_64.tar.gz" ;;
  Darwin:x86_64) asset="lizard-macos-x86_64.tar.gz" ;;
  *)
    printf 'Unsupported platform: %s/%s\n' "$os_name" "$arch_name" >&2
    exit 1
    ;;
esac

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

mkdir -p "$install_dir"
curl --fail --location --silent --show-error "$release_base_url/$asset" -o "$tmp_dir/$asset"
tar -xzf "$tmp_dir/$asset" -C "$tmp_dir"
package_dir="$(find "$tmp_dir" -type d -name 'lizard-*' -print -quit)"
install -m 755 "$package_dir/lz" "$install_dir/lz"
install -m 755 "$package_dir/lizard" "$install_dir/lizard"

printf 'LIZARD installed in %s\n' "$install_dir"
printf 'Add %s to PATH if it is not already present.\n' "$install_dir"
"$install_dir/lz" --version
