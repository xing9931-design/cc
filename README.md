# wclean

A safe, fast **C: drive cleanup tool for Windows**, written in Rust.

`wclean` reclaims disk space from the usual suspects — temporary files, browser
caches and the Recycle Bin — and can point you at the largest files on a drive
so you can decide what else to remove. It is a command-line tool with colored
output, and every destructive action is **opt-in and confirmed**.

## Features

| Category            | What it cleans                                                        |
| ------------------- | -------------------------------------------------------------------- |
| Temporary files     | `%TEMP%`, `%TMP%`, `C:\Windows\Temp`, and `Prefetch`                  |
| Browser caches      | Chrome, Edge, Brave and Firefox caches (all profiles)                |
| Recycle Bin         | Empties the Recycle Bin across all drives (Win32 Shell API)          |
| Large file scan     | Reports the biggest files under a path — **report only, never deletes** |

### Safety first

- `scan` and `--dry-run` never touch the disk — they only measure.
- `clean` shows you exactly what it found and **asks for confirmation** before
  deleting (skip with `--yes` for automation).
- Files that are locked or in use are skipped, never forced — wclean reports how
  many it had to leave behind.
- The large-file scanner only *lists* files. It will never delete your data.

## Install / Build

Requires a [Rust toolchain](https://rustup.rs/). Build a release binary:

```sh
cargo build --release
# binary at: target/release/wclean.exe  (Windows)
```

The Recycle Bin feature uses Win32 Shell APIs and only works on Windows. The
project still builds and tests run on Linux/macOS (the Recycle Bin becomes a
no-op), which keeps development possible off-Windows.

## Usage

```sh
# See how much space you could reclaim (nothing is deleted)
wclean scan

# Scan only specific categories
wclean scan temp browser

# Preview a clean without deleting
wclean clean --dry-run

# Clean everything, with a confirmation prompt
wclean clean

# Clean without prompting (for scripts / scheduled tasks)
wclean clean --yes

# Clean only the Recycle Bin
wclean clean recyclebin --yes

# Find the 20 largest files on C: that are at least 100 MB
wclean large

# Find big files under a specific folder
wclean large "D:\Downloads" --min-mb 50 --top 30
```

### Categories

`temp`, `browser`, `recyclebin` — pass any subset to `scan`/`clean`, or none to
act on all of them.

## Notes

- Some system temp files and an in-use Recycle Bin may require running the tool
  from an **elevated (Administrator)** prompt to fully clean.
- Close your browser before cleaning its cache so locked files can be removed.

## Project layout

```
src/
  main.rs            CLI entry point and command flow
  cli.rs             Argument parsing (clap)
  ui.rs              Terminal output, prompts, formatting
  util.rs            Byte formatting, directory sizing, safe deletion
  cleaner/
    mod.rs           Category definitions and dispatch
    temp.rs          Temporary-file locations
    browser.rs       Browser cache discovery (Chrome/Edge/Brave/Firefox)
    recyclebin.rs    Recycle Bin via Win32 Shell API
    largefiles.rs    Bounded-memory large-file scanner
```

## License

MIT
