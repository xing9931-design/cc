# wclean — Product Brief

> The thinking behind *what* we build and *why*. Companion to
> [`DESIGN.md`](DESIGN.md) (how it looks and feels).

## Problem

Windows users run out of space on `C:`. The drive fills with caches and
temporary files they can't see and don't understand. The built-in Disk Cleanup
is slow and opaque; third-party "cleaners" are bloated, ad-laden, and erode
trust by over-promising and over-deleting.

## Who it's for

- **Primary — "My C: drive is full."** A non-technical Windows user who sees the
  red bar in Explorer and wants it gone, safely, without learning what a cache
  is. Needs confidence, not knobs.
- **Secondary — "I want control."** A power user who wants a fast, scriptable,
  transparent tool with no telemetry and no surprises. Uses the CLI.

## Principles

1. **Safety is the product.** We never delete user data. We delete only
   regenerable junk, we *preview before we act*, and we *confirm before we
   delete*. The large-file finder only ever reports. A cleaner people don't
   trust is worthless, however much space it frees.
2. **See before you sweep.** Understanding precedes action. The first thing you
   see is how full your drive is and exactly what can be reclaimed — not a
   one-click black box.
3. **Honest and quiet.** No ads, no telemetry, no nagging, no "PC is at risk!"
   dark patterns. Plain language over jargon.
4. **Fast and small.** Rust, async scanning, a single dependency-light binary.
   The tool should feel instant and weigh almost nothing.
5. **Respect, don't override.** Locked/in-use files are skipped and reported,
   never force-killed. We explain when elevation is needed instead of demanding
   it up front.

## What it does (v0.2)

| Capability        | Behavior                                                       |
| ----------------- | -------------------------------------------------------------- |
| Temporary files   | Clean `%TEMP%`, `Windows\Temp`, `Prefetch` — **Safe**         |
| Browser caches    | Chrome / Edge / Brave / Firefox, all profiles — **Low impact** |
| Recycle Bin       | Empty via the Win32 Shell API — **Low impact**                 |
| Large-file finder | Rank the biggest files under a folder — **report only**        |
| Drive gauge       | Live `C:` usage so the user sees the problem and the result    |

Every category carries a **risk label** so the user always knows what's safe.

## Deliberately *not* in scope (and why)

- **Registry cleaning** — high risk, negligible benefit, classic snake-oil.
  Off-brand for a trust-first product.
- **Automatic background cleaning** — surprise deletion is the opposite of
  trust. Cleaning is always user-initiated and confirmed.
- **Telemetry / accounts / "pro" upsells** — privacy is a feature.
- **Uninstaller / startup manager** — scope creep; Windows already has these.

## Success looks like

- A first-run user reclaims space in **under 30 seconds** without reading docs.
- **Zero** reports of lost data — the bar that actually matters.
- The user *understands* what happened: "85% → freed 4.2 GB" is legible at a
  glance.

## Roadmap (candidates)

- Per-category drill-down ("show me what's in here") before deleting.
- Windows Update / Delivery Optimization cache, thumbnail cache, dump files.
- Disk-usage treemap for the large-file view.
- Remembered preferences (theme, default selection) and a scheduled, *opt-in*
  reminder (never an automatic delete).
- Signed installer + auto-update.
