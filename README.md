# Insulator

Insulator is a native desktop app for running and managing local coding agents. It is built with Rust and [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui). Insulator is a fork of [insulator.sh](https://insulator.sh/) and is proudly built on its shoulders. Projects, sessions, transcripts, and app data stay on your machine. No Insulator account or hosted service is required.

## Features

- Manage multiple projects and independent agent sessions.
- Use one interface for models, reasoning effort, access modes, and follow-up messages.
- Queue or steer messages while an agent is working.
- Rewind Git-backed work with conversation-aware checkpoints.
- Review files, diffs, skills, usage, attachments, and task state.
- Connect to a standalone daemon when the agent should run on another machine.

## Supported agents

- [Amp](https://ampcode.com/)
- Claude Code
- Codex CLI
- Cursor CLI
- [Fx](https://fx.sh/)
- Grok Build
- Kimi Code
- OpenCode
- Pi

Install and authenticate at least one supported agent CLI before launching Insulator. Insulator detects installed CLIs automatically and uses their native protocols and session continuity.

## Install

- **macOS:** Download the signed DMG from the [latest GitHub release](https://github.com/egoist/insulator/releases/latest).
- **Linux:** See [docs/linux.md](docs/linux.md) for current installation options.
- **Windows:** Download the latest [installer](https://github.com/egoist/insulator/releases/latest). A portable ZIP is also available. See [docs/windows.md](docs/windows.md).

The embedded browser and computer-use integration are currently macOS-only.

## Development

Requirements: Rust 1.96+ and [Bun](https://bun.sh/).

```sh
bun install
bun run dev
```

The development watcher rebuilds and relaunches Insulator Debug. To update the browser protocol types after changing a Rust wire type:

```sh
bun run protocol:generate
bun run protocol:check
```

See [RELEASING.md](RELEASING.md) for release instructions.

## License

[GNU General Public License v3.0 only](LICENSE)
