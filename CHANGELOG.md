# Changelog

All notable changes to wclean are documented here. This project adheres to
[Semantic Versioning](https://semver.org/).

## [0.6.0]

### Added

- **Storage breakdown** — analyze any folder and see what's using the space as
  ranked, proportional bars ("Users — 45%"). Answers "where did my space go?",
  not just "what junk can I delete?" (`usage` module).
- The GUI's large-file area is now a unified **"Explore your disk"** section
  with two lenses on the same folder: *Storage breakdown* and *Largest files*.
- New CLI command: `wclean breakdown [path] [--top N]`.
- A painted proportional-bar widget for the breakdown rows.

### Changed

- `largefiles::scan` is reused via `scan_roots`; results rendering refactored
  into `breakdown_results` / `large_results`.

## [0.5.0]

### Added

- **Category drill-down** — after a scan, expand any folder-based category to
  preview the largest individual files it would remove, fetched on a background
  thread (`cleaner::detail` over `largefiles::scan_roots`). Transparency before
  deletion.
- **Lifetime stats** — the GUI remembers and displays how much you've reclaimed
  in total and how many cleanups you've run ("★ 12.6 GB reclaimed over 7
  cleanups"), persisted with your preferences.
- **Keyboard shortcuts** — Enter runs the primary action (scan, then open the
  confirm dialog); Esc cancels the dialog; Enter confirms it.

### Changed

- `largefiles::scan` now delegates to a new `scan_roots` that scans multiple
  directories with one bounded heap.
- Category directory sets are exposed (`cleaner::category_dirs`) to power the
  drill-down.

## [0.4.0]

### Added

- **Preference persistence** (`config` module, dependency-free `key=value`
  file): the GUI remembers your theme, category selection, and large-file
  settings between launches. Stored under `%APPDATA%\wclean\config`.
- **Reveal in file manager** — large-file results gain a *Reveal* action that
  highlights the file in Explorer (Windows), Finder (macOS) or the containing
  folder (Linux).
- **Select all / Clear** quick controls for the category list.
- **App icon** generated in code (the brand mark) for the window and taskbar —
  no image asset shipped.

### Changed

- Theme resolution order is now: `WCLEAN_THEME` env override → saved preference
  → dark default.
- Stable category keys (`Category::key` / `from_key`) underpin persistence.

## [0.3.0]

### Added

- **Two new cleanup categories**: Windows Update download cache
  (`SoftwareDistribution\Download`) and crash dumps / Windows Error Reporting
  (`system` module), each with a risk label.
- **Animated drive gauge** — the ring fill sweeps in and the percentage counts
  up via egui animation curves.
- **Release workflow** (`.github/workflows/release.yml`) that builds the CLI and
  GUI `.exe`s on Windows and attaches them to a GitHub Release on version tags.
- More unit tests covering the system-junk scan/clean logic.

### Changed

- GUI selection state is now dynamic (`Vec<bool>`), adapting to any number of
  categories; default window height increased for the longer list.

## [0.2.0]

### Added

- **Redesigned GUI** built on a proper design system (`src/gui/theme.rs`):
  color/spacing/radius/type tokens, light **and** dark themes (runtime toggle,
  `WCLEAN_THEME` override).
- **Drive-usage hero** — a painted ring gauge showing live `C:` usage, with the
  fill color shifting accent → amber → red as the disk fills.
- **Category cards** with icons, plain-language descriptions, and **risk badges**
  (Safe / Low impact); the whole card is the selection target.
- **Disk capacity** querying via `GetDiskFreeSpaceExW` (`diskinfo` module), with
  graceful off-Windows fallback.
- Product and design documentation under `docs/` (`PRODUCT.md`, `DESIGN.md`).

### Changed

- Core cleanup logic extracted into a reusable **library crate** (`wclean`),
  consumed by both the CLI and GUI binaries.
- The GUI now lives in `wclean::gui` (theme / widgets / app modules); the
  `wclean-gui` binary is a thin shim.
- CI lints and builds all features and builds the GUI on Windows.

## [0.1.0]

### Added

- Initial CLI: temporary-file, browser-cache and Recycle Bin cleanup, plus a
  bounded-memory large-file scanner.
- Safety model: scan / `--dry-run` never touch disk; `clean` confirms unless
  `--yes`; locked files are skipped and reported.
- First GUI (egui) and CI (fmt + clippy + build/test on Windows and Linux).
