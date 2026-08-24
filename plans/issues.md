# Issues and Questions

Living list for the v1 driver-layer rewrite. Not a bug tracker.

## Issues

_None yet._

## Questions

### Q-001: Grok non-interactive surface

| Field | Value |
|-------|-------|
| Status | Answered |
| Discovered | 2026-08-24 rewrite |
| Priority | High |

The Grok/Grok Build CLI's non-interactive entry (`-p`, `--prompt`, `exec`) varies by version. The Grok adapter should probe the installed binary rather than freeze a flag set in the shared contract. Confirm against the binary shipped to users before locking the adapter tests.

**Answer (v1):** The Grok adapter probes `--help` at map time (not detect time — `occam drivers` must not invoke the CLI). Preference order: `exec`, then `--prompt`, then `-p`. The shared driver contract does not freeze Grok flags.

### Q-002: How far can `detect()` see authentication?

| Field | Value |
|-------|-------|
| Status | Answered |
| Discovered | 2026-08-24 rewrite |
| Priority | Medium |

Some CLIs have a cheap “am I logged in?” check; others only fail once invoked. Prefer detect-time `unauthenticated` (exit `4`) when it is reliable. Otherwise invoke and map the driver's auth error to `4`. Do not scrape credentials.

**Answer (v1):** `detect()` is binary-on-PATH. Auth is mapped from driver stderr after invoke (needles such as “not logged in”, “authentication required”). Exit `4` points at the CLI's own login (`codex login`, `claude /login`, Grok CLI login). No credential scraping.

### Q-003: Default timeout value

| Field | Value |
|-------|-------|
| Status | Answered |
| Discovered | 2026-08-24 rewrite |
| Priority | Low |

Design currently suggests 120s when unset so pipelines cannot hang forever. Confirm against typical `review` / `diagnose` runtimes before shipping. Tasks may always override.

**Answer (v1):** Default timeout is `120s` (config, else this). Tasks and `--timeout` override. Conservative on purpose: a stuck agent must not hang a pipeline.
