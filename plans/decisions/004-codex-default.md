# ADR 004 — Codex is the default driver

| Field | Value |
|-------|-------|
| Status | Accepted |
| Date | 2026-08-24 |

## Context

Scripts need a default when `--driver`, task-level driver, `OCCAM_DRIVER`, and config are all unset. Making “whatever is installed” the default would make exit codes and capability behaviour surprising.

## Decision

Default driver id is `codex`.

Resolution order: `--driver` → task `driver` → `OCCAM_DRIVER` → `default_driver` in config → `codex`.

Optional `drivers = ["codex", "claude"]` is **unavailable-fallback only**: used when the chosen driver is missing or unusable at detect time. It is not quality-based routing.

## Consequences

- A machine with only Claude installed must set config, env, or flags (or a fallback list).
- `occam drivers` exists so this is discoverable without a failed run.
