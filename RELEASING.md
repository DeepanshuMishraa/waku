# Releasing Insulator

Insulator's macOS appcast and update archives are published as GitHub Release assets.
The release workflow creates a `v<version>` tag and release title (for example, `v0.2.0`);
Sparkle reads the appcast from the latest release. macOS builds use ad-hoc code signing
because this distribution does not use a Developer ID certificate. Sparkle still verifies
archives with its Ed25519 signing key.

Insulator ships signed in-app updates on macOS, Linux, and Windows. Releases live in
a **Cloudflare R2** bucket served at **`https://releases.insulator.sh`**. macOS uses
[Sparkle](https://sparkle-project.org), including binary deltas when available;
the native Linux and Windows updaters read architecture-specific feeds and
verify artifacts with the same EdDSA key. One release workflow produces all
platform artifacts and feeds.

Once set up, cutting a release is:

```sh
bun run release
```

- Updater code: [`src/updater.rs`](src/updater.rs) — loads the embedded
  Sparkle.framework on macOS and owns the signed native flows on Linux and
  Windows. Available updates appear in the sidebar footer; **Check for
  Updates…** lives in the app menu, and **Automatic updates** lives in
  Settings → General.
- Feed URL + public key: [`resources/Info.plist`](resources/Info.plist)
  (`SUFeedURL`, `SUPublicEDKey`).
- Framework embedding + pinned Sparkle version:
  [`scripts/bundle.sh`](scripts/bundle.sh) (bump `sparkle_version` and
  `sparkle_sha256` together; the distribution is cached under
  `.insulator-cache/sparkle/`).
- Release automation: [`scripts/release.ts`](scripts/release.ts),
  [`scripts/appcast.ts`](scripts/appcast.ts),
  [`scripts/changelog.ts`](scripts/changelog.ts).
- GitHub Actions: [`.github/workflows/release.yml`](.github/workflows/release.yml)
  builds Linux (x86_64, arm64), Windows (x86_64, arm64), and macOS archives
  automatically when a `v*` release is published;
  [`.github/workflows/sync-release.yml`](.github/workflows/sync-release.yml)
  copies published assets into the R2 bucket.

---

## One-time setup

The release runs on [Bun](https://bun.sh) and needs
[`create-dmg`](https://github.com/create-dmg/create-dmg) and
[rclone](https://rclone.org) (`brew install bun create-dmg rclone`).

### 1. Sparkle signing keys

Updates are signed with an ed25519 key; the private half stays in the login
keychain and the public half ships in Info.plist as `SUPublicEDKey`.

**This Mac already has the key** — Insulator signs with the same default-account
Sparkle key as kero, and the matching public key is already in Info.plist.
Nothing to do.

On a fresh machine, restore the key from the password-manager backup with the
Sparkle tools (they land in `.insulator-cache/sparkle/<version>/bin` after any
build, or download the release from
[sparkle-project/Sparkle](https://github.com/sparkle-project/Sparkle/releases)):

```sh
./bin/generate_keys -f sparkle_private_key.txt   # import the backed-up key
./bin/generate_keys -p                            # prints the public key — must
                                                  # match SUPublicEDKey
```

> ⚠️ Lose the private key and existing installs can never update again. Keep
> the backup current.

To split Insulator onto its own key later: `generate_keys --account insulator`, put the
new public key in Info.plist, and pass `--account insulator` through to
`generate_appcast` in `scripts/appcast.ts`. Users on old builds only trust the
old key, so do this on a release that still signs with the old key… in other
words, don't do it casually.

### 2. Release build configuration

No analytics configuration is required. GitHub Actions uses
`bun run release --local --adhoc`, so a Developer ID certificate and notarization
profile are not required. For a separately notarized local build, configure the
signing identity and `NOTARY` keychain profile as usual.

### 3. Cloudflare R2 bucket + domain  ← **still to do once**

1. Create the bucket **`insulator-releases`** (Cloudflare dashboard → R2 → Create
   bucket). The release script will not create it — a bucket-scoped API token
   can't.
2. Attach the custom domain **`releases.insulator.sh`** to the bucket (bucket →
   Settings → Custom Domains). This serves objects publicly at
   `https://releases.insulator.sh/<file>`.
3. Make sure the R2 API token behind the `r2` rclone remote covers this bucket
   (R2 → Manage API Tokens → Object Read & Write). The remote already exists
   for kero; if `rclone lsf r2:insulator-releases --s3-no-check-bucket` returns
   *AccessDenied* after the bucket exists, extend the token's bucket list.

The rclone remote itself (`~/.config/rclone/rclone.conf`, type S3, provider
Cloudflare, `no_check_bucket = true`) is shared with kero and needs no change.

---

## Cutting a release

1. **Bump `version` in `Cargo.toml`** — the single source of truth.
   `CFBundleShortVersionString` is the version, and `CFBundleVersion` is
   derived from it (`major*1e6 + minor*1e3 + patch`, so `0.2.0` → `2000`),
   which keeps Sparkle's build-number comparison monotonic without a manual
   counter. Prerelease versions (`-beta.1`) are refused for publishing — the
   appcast serves one stable channel.
2. **Write the release notes** — add a `## [<version>]` section at the top of
   [`CHANGELOG.md`](CHANGELOG.md).
3. **Run it:**
   ```sh
   bun run release
   ```

The script checks R2 up front (bucket reachable, version not already
published), builds and signs the app via `scripts/bundle.sh release`, verifies
the bundled JS REPL and computer-use helper, builds the styled DMG, notarizes
and staples DMG + app, zips the app for Sparkle, pulls the recent archives
from R2 so `generate_appcast` can build binary deltas, attaches the changelog
section as release notes, regenerates the signed `appcast.xml`, and uploads
everything with immutable cache headers (the appcast itself stays
`max-age=300`). When it finishes:

- **Download link**: `https://releases.insulator.sh/Insulator-<version>.dmg`
- **In-app updates**: served from the same origin via the appcast.

Test by keeping an older build around, launching it, and choosing
**Check for Updates…**.

### GitHub release + R2 sync

The release flow is:

1. Bump `version` in `Cargo.toml`.
2. Create and publish a GitHub release with a matching `v<version>` tag.

Publishing the release triggers the workflow automatically. It validates the tag against
`Cargo.toml`, builds every platform, extracts
that version's `CHANGELOG.md` section into the release body, and attaches the DMG,
ZIP, appcast, and other platform assets to the existing release.

macOS CI runs `bun run release --local --adhoc`, which writes the
same artifacts as a local release:

- `Insulator-<version>.dmg`
- `Insulator-<version>.zip`
- `appcast.xml` (Sparkle-signed)

Linux CI adds:

- `insulator-<version>-x86_64-unknown-linux-gnu.tar.gz`
- `insulator-<version>-aarch64-unknown-linux-gnu.tar.gz`
- `appcast-linux-x86_64.xml`, `appcast-linux-aarch64.xml`
- `latest-linux.txt` — the version `install.sh` resolves "latest" to

Windows CI adds:

- `Insulator-<version>-x86_64-Setup.exe`
- `Insulator-<version>-aarch64-Setup.exe`
- `insulator-<version>-x86_64-pc-windows-msvc.zip` (portable)
- `insulator-<version>-aarch64-pc-windows-msvc.zip` (portable)
- `appcast-windows-x86_64.xml`, `appcast-windows-aarch64.xml`
- `latest-windows.txt` — the version the download page resolves "latest" to

[`scripts/bundle-windows.ts`](scripts/bundle-windows.ts) builds both, driving
[`resources/windows/insulator.iss`](resources/windows/insulator.iss) through Inno Setup's
`ISCC`. The installer is **per-user** (`PrivilegesRequired=lowest`,
`%LOCALAPPDATA%\Programs\Insulator`) — no elevation, which is exactly what lets the
updater re-run it silently. The script signs the two executables and the
installer with Authenticode when `WINDOWS_CERTIFICATE` and
`WINDOWS_CERTIFICATE_PASSWORD` are set, and packages them unsigned otherwise,
so a fork without a certificate can still cut a release at the cost of a
SmartScreen warning.

**Never change `AppId` in `insulator.iss`.** It is how Windows recognizes an
existing install; a new one turns every update into a second copy in
Add/Remove Programs.

#### The native Windows and Linux update feeds

Windows and Linux have no Sparkle, so [`src/updater.rs`](src/updater.rs) runs
the same contract itself: fetch the appcast, compare versions, download, and
verify the EdDSA signature. Windows hands the installer to Inno Setup with
`/SILENT`. Linux safely unpacks the tarball beside the managed user-local
prefix, then `insulator-updater` swaps it after the app's normal quit saves and
rolls back if the replacement cannot open its main window.

- **One feed per architecture.** A Sparkle appcast cannot say which binary an
  item is for, and the client picks its feed at compile time.
- **Same key as macOS.** `build.rs` reads `SUPublicEDKey` out of
  `resources/Info.plist` and compiles it in, so the three platforms cannot
  drift onto different keys.
- [`scripts/appcast-windows.ts`](scripts/appcast-windows.ts) and
  [`scripts/appcast-linux.ts`](scripts/appcast-linux.ts) sign the feeds in the
  release asset job — the only one holding all native artifacts. They sign
  with Node's Ed25519 over the same `SPARKLE_PRIVATE_KEY`, and refuse to run
  when the key does not derive `SUPublicEDKey` (signing with the wrong key
  ships a feed the app rejects).
- The step pulls the live feeds down first and merges, so previously published
  releases keep their entries.

Both Linux jobs run on **Ubuntu 22.04**, and that choice is load-bearing: the
binaries link against the build machine's glibc, so the runner sets the oldest
distribution Insulator can start on (2.35 — Ubuntu 22.04, Debian 12, Fedora 36).
Moving those jobs to a newer runner silently drops support for everything
older.

The workflow updates the published GitHub release with those files and
the matching `CHANGELOG.md` section. Publishing the release also syncs the
assets, including every signed update feed, to R2.

`appcast.xml`, the architecture-specific Linux/Windows appcasts,
`latest-linux.txt`, and `latest-windows.txt` are the bucket's mutable pointers
and upload with a short cache lifetime; everything else is versioned and
cached forever. Linux users install from that bucket via
[`website/public/install.sh`](website/public/install.sh), served at
`https://insulator.sh/install.sh` — see [docs/linux.md](docs/linux.md).

Publishing that GitHub release (or running **Sync release** from Actions)
uploads the assets to the `insulator-releases` R2 bucket. Configure these repository
secrets first:

| Secret | Purpose |
| --- | --- |
| `INSULATOR_SIGNING_IDENTITY` | Developer ID identity selector |
| `APPLE_CERTIFICATE` | base64-encoded Developer ID Application `.p12` |
| `APPLE_CERTIFICATE_PASSWORD` | password for that `.p12` |
| `APPLE_ID` | Apple ID used by `notarytool` |
| `APPLE_APP_SPECIFIC_PASSWORD` | app-specific password for that Apple ID |
| `APPLE_TEAM_ID` | Developer Team ID |
| `SPARKLE_PRIVATE_KEY` | EdDSA private key for `generate_appcast` |
| `WINDOWS_CERTIFICATE` | optional; base64-encoded Authenticode `.pfx` |
| `WINDOWS_CERTIFICATE_PASSWORD` | optional; password for that `.pfx` |
| `R2_ACCOUNT_ID` | Cloudflare account id for the R2 API |
| `R2_ACCESS_KEY_ID` | R2 Object Read & Write token |
| `R2_SECRET_ACCESS_KEY` | matching secret |
| `R2_BUCKET` | optional; defaults to `insulator-releases` |

### Options

| Flag / Env | Default | Purpose |
| --- | --- | --- |
| `--local` | — | build, notarize, and write the DMG + zip without publishing |
| `--force` | — | re-publish a version that already exists in R2 |
| `--adhoc`, `--skip-notarize` | — | local test builds (imply `--local`) |
| `--skip-build` | — | reuse existing release binaries |
| `--build-number <n>` / `INSULATOR_BUILD_NUMBER` | derived | `CFBundleVersion` override |
| `INSULATOR_R2_REMOTE` | `r2` | rclone remote name |
| `INSULATOR_R2_BUCKET` | `insulator-releases` | R2 bucket |
| `INSULATOR_DOWNLOAD_URL_PREFIX` | `https://releases.insulator.sh/` | base URL in the appcast |
| `INSULATOR_HISTORY_COUNT` | `15` | recent archives pulled for delta generation |
| `INSULATOR_NO_HISTORY=1` | — | skip pulling old archives (full updates only) |
| `SPARKLE_BIN` | the `.insulator-cache` copy | Sparkle tools directory |

---

## Notes

- **Two artifacts per release:** the notarized `.dmg` (what people download)
  and a `.zip` (what Sparkle installs, plus `.delta` files against recent
  builds). Only the zip family appears in the appcast; point download buttons
  at the DMG.
- **Debug builds never update themselves.** `Updater::init` returns `None`
  under `debug_assertions`, so the dev watcher's app can't offer to replace
  itself with a production Insulator. Set `INSULATOR_FORCE_UPDATER=1` to exercise the
  real Sparkle flow from a debug bundle anyway. A bare `cargo run` binary has
  no embedded framework and also degrades to no updater. For UI-only testing,
  start the watcher with `INSULATOR_PREVIEW_UPDATE=1`; the sidebar immediately
  shows an available update and clicking it changes to the spinner without
  installing anything. The preview flag fakes only that sidebar result;
  **Check for Updates…** still uses the embedded Sparkle framework and its
  real standard window.
- **Automatic and explicit checks have separate presentation.** Scheduled
  checks stay silent until the sidebar update button appears. Choosing
  **Check for Updates…** promotes an existing silent result into Sparkle's
  standard updater window, or shows its checking progress while an automatic
  check finishes. With no automatic session active, it starts Sparkle's
  standard user-initiated check directly.
- **First-run consent:** Sparkle shows its one-time "check automatically?"
  prompt on the second launch. The Settings → General toggle reads and writes
  the same persisted value.
- **Insulator isn't sandboxed**, so Sparkle's XPC services are unnecessary;
  `bundle.sh` strips them (plus headers/modules) from the embedded framework
  and re-signs the rest with the app's identity — hardened-runtime library
  validation requires the identities to match.
- **Old archives stay in R2** so far-behind users can still be served; only
  the recent history is staged locally under `dist/updates/` (git-ignored).
- **Platform artifacts:** keep the bucket layout flat and platform-tagged by
  artifact name/extension — today's macOS names
  (`Insulator-<v>.dmg`, `Insulator-<v>.zip`, `appcast.xml`) must keep their URLs.
  Linux CI releases produce `insulator-<v>-<target>.tar.gz` with
  `scripts/bundle-linux.sh`, Windows CI produces `insulator-<v>-<target>.zip` with
  `scripts/bundle-windows.ts`, and both land in GitHub Releases, then R2 via
  the sync workflow. Windows also ships `Insulator-<v>-<arch>-Setup.exe`; each
  native client updates from `appcast-<platform>-<arch>.xml` while the Linux
  installer resolves `latest-linux.txt`. `src/updater.rs` is the per-platform
  seam, and everything
  mac-specific in the existing release pipeline lives behind the Darwin guard
  in `scripts/release.ts` plus `scripts/bundle.sh`.
