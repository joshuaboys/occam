# Occam v1

| Field   | Value              |
|---------|--------------------|
| Status  | Ready              |
| Owner   | Josh Boys          |
| Created | 2026-03-15         |
| Revised | 2026-08-24         |

Design: [2026-08-24-occam-v1](../designs/2026-08-24-occam-v1.design.md)

## Overview

Occam is a stateless Unix-style execution layer for bounded, single-shot agent work. It drives coding-agent CLIs the user already has installed and authenticated (`codex`, `claude`, `grok`). It is not an agent framework and not a prettier wrapper around one vendor's flags.

## Problem & Success Criteria

**Problem:** Direct `claude -p` / `codex exec` / `grok -p` invocations are not a portable contract. Stdin, stdout, bounds, JSON, and exit codes differ per CLI and per machine. The previous Occam spec answered this by owning providers, tools, and a Bun runtime — competing with the CLIs instead of bounding them.

**Success Criteria:**

- [ ] `occam <task>` runs a named task against a local driver and exits
- [ ] Piped stdin is task input; child stdin is never accidentally inherited
- [ ] stdout is the task result; stderr is diagnostics
- [ ] Timeout kills the process group and returns exit `6`
- [ ] Schema tasks emit valid JSON or fail with exit `7` (never invalid JSON + `0`)
- [ ] `--driver` / `OCCAM_DRIVER` / config switch Codex, Claude, and Grok without changing the task
- [ ] Missing driver → exit `3`; unauthenticated driver → exit `4` pointing at that CLI's login
- [ ] Single Rust binary; no provider SDKs, API keys, or Occam-owned tool loop
- [ ] No persistent state, memory, or cross-run dependencies

## Constraints

- Rust implementation. Bun/TypeScript (D-001) is superseded.
- TOML for config and task definitions — not a workflow DSL
- Stateless: no run history, no memory, no retained state
- Single-run: one child process, then terminate — no daemons
- No multi-agent delegation, orchestration, or Occam-native tools
- Auth is the underlying CLI's. Occam never stores provider credentials
- Unix composability is a hard requirement
- Do not enforce a driver capability the driver does not expose

## Modules

| Module | Purpose | Status | Dependencies |
|--------|---------|--------|--------------|
| [CLI](./modules/01-cli.aps.md) | Invocation surface, flags, signals, process exit | Ready | CFG, TSK, DRV, RUN, OUT |
| [CFG](./modules/02-config.aps.md) | Config, env, driver command paths, fallback | Ready | — |
| [TSK](./modules/03-task.aps.md) | Named task definitions | Ready | CFG |
| [DRV](./modules/04-driver.aps.md) | Local CLI drivers: detect, capabilities, invoke | Ready | CFG |
| [RUN](./modules/05-run.aps.md) | One child-process lifecycle and external limits | Ready | DRV, TSK |
| [OUT](./modules/06-output.aps.md) | stdout/stderr, schema, envelope, exit codes | Ready | — |

## Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Occam collapses into a flag alias | High | Product test on every feature; acceptance criteria in the design |
| Driver flags drift per vendor release | High | Adapters probe and map; unsupported → exit `8`; do not freeze vendor flags in the shared contract |
| Tool loop sneaks back in | High | ADR 003; drivers own tools |
| Task TOML grows into a workflow DSL | Medium | Reject orchestration fields; keep the allowed key list closed |
| Fallback becomes model routing | Medium | Fallback only on detect-unavailable |
| Schema repair loops | Medium | Exactly one repair attempt, then exit `7` |

## Open Questions

Answered or withdrawn in the 2026-08-24 rewrite. Remaining items live in [issues.md](./issues.md).

## Decisions

- **D-001 (superseded):** Bun as runtime — **void**. Replaced by [ADR 001](./decisions/001-rust.md) (Rust).
- **D-002 (kept, narrowed):** TOML for config and tasks, not agent personas.
- **D-003 (strengthened):** No agent SDK as core. No provider SDKs at all. [ADR 002](./decisions/002-driver-not-provider.md).
- **D-004 (kept):** Stateless by design.
- **D-005:** Codex is the default driver. [ADR 004](./decisions/004-codex-default.md).
- **D-006:** No Occam-native tool loop. [ADR 003](./decisions/003-no-native-tool-loop.md).
