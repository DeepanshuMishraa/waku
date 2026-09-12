# Changelog

All notable changes to Insulator. This file is the **source of truth for the release
notes shown in the in-app updater**: [`scripts/release.ts`](scripts/release.ts)
extracts the section whose heading matches the version being released
(`MARKETING_VERSION`) and publishes it next to the update, so Sparkle shows it in
the update prompt.

Format follows [Keep a Changelog](https://keepachangelog.com). Add a new
`## [<version>]` section at the top for each release, matching the version you
set in the Xcode project.

Write release notes for the final product users receive, not the development
history. When a feature is still unreleased, fold its fixes and refinements into
the original feature bullet instead of adding separate entries for them.

## [unreleased]

## [0.1.4]

- Add a Transparent window style with native macOS vibrancy and live window transparency controls
- Keep transparent surfaces, dialogs, pickers, and panels readable as transparency changes
- Keep sidebar and main canvas on the same blur layer, with consistent styling at every transparency level
- Add configurable sidebar transparency and a darker canvas for clearer agent workspaces
- Refresh sidebar, new-task, settings, and supporting UI icons
- Expose the composer to macOS accessibility tools
- Add Intel macOS release builds

## [0.1.3]

- Keep popovers, context menus, and pickers opaque in Liquid Glass and Image window styles for readability
- Fix settings view background opacity in Image and Liquid Glass window styles
- Add file editor autosave and immediate Command-S (`⌘S`) save with toast feedback
- Show Pi subagents in Activity with one row per running agent, start notifications, and Stop/Delete controls
- Fix duplicate subagent entries and garbled ANSI completion toasts
