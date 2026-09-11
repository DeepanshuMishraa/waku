#!/usr/bin/env bash

set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

target_dir="${CARGO_TARGET_DIR:-target}"
version="$(cargo metadata --no-deps --format-version 1 | sed -n 's/.*"name":"insulator","version":"\([^"]*\)".*/\1/p')"
target_triple="$(rustc -vV | sed -n 's/^host: //p')"
package="insulator-${version}-${target_triple}"
archive="$target_dir/release/$package.tar.gz"
staging="$(mktemp -d)"
trap 'rm -rf -- "$staging"' EXIT

cargo build --locked --release \
  --package insulator --bin insulator --bin insulator-updater \
  --package insulator-daemon --bin insulator-daemon

package_dir="$staging/$package"
install -Dm755 "$target_dir/release/insulator" "$package_dir/bin/insulator"
install -Dm755 "$target_dir/release/insulator-updater" "$package_dir/bin/insulator-updater"
install -Dm755 "$target_dir/release/insulator-daemon" "$package_dir/bin/insulator-daemon"
install -Dm644 resources/linux/sh.insulator.desktop \
  "$package_dir/share/applications/sh.insulator.desktop"
install -Dm644 resources/linux/self-update-v1 \
  "$package_dir/share/insulator/self-update-v1"
install -Dm644 website/public/app-icon.png \
  "$package_dir/share/icons/hicolor/256x256/apps/sh.insulator.png"
install -Dm644 LICENSE "$package_dir/share/licenses/insulator/LICENSE"

mkdir -p "$(dirname "$archive")"
tar -C "$staging" -czf "$archive" "$package"
printf 'Created %s\n' "$archive"
