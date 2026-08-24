# Drivers

| ID | Owner | Status |
|----|-------|--------|
| DRV | Josh Boys | Complete |

## Purpose

A driver is a locally installed executable coding-agent CLI. Drivers detect usability, advertise capabilities, and map a driver-neutral `RunRequest` onto that CLI's non-interactive invocation. Occam does not call model APIs.

## In Scope

- Driver ids: `codex` (default), `claude`, `grok`
- `detect`: missing / unauthenticated / ready, where feasible
- Capability advertisement: `structured_output`, `schema_output`, `streaming_output`, `max_turns`, `tool_allowlist`, `sandbox_control`, `ephemeral_mode`
- Mapping RunRequest onto argv (and controlled stdin) per adapter
- Login hints that point at the underlying CLI, never an Occam login
- Configurable `command` path

## Out of Scope

- Process timeout and kill (→ RUN)
- Result schema validation (→ OUT)
- Provider SDKs, API keys, OAuth, token refresh
- Occam-owned tools or MCP
- Account isolation (leave to the user's CLI or an external wrapper)
- Sophisticated routing among models

## Interfaces

**Depends on:**

- CFG — command path and fallback list

**Exposes:**

- Driver trait (id, detect, capabilities, run mapping) to RUN and CLI
- `occam drivers` rows (id, command, status, capabilities)

## Ready Checklist

Change status to **Ready** when:

- [x] Purpose and scope are clear
- [x] Dependencies identified
- [x] At least one task defined

## Work Items

### DRV-001: Driver trait and detection

- **Intent:** Each bundled driver can be listed and detected without invoking a model.
- **Expected Outcome:** Missing binaries report unavailable; ready binaries report ready; `occam drivers` is complete for `codex`, `claude`, `grok`.
- **Validation:** `cargo test -q --test driver_detect`
- **Status:** Complete

### DRV-002: Codex, Claude, and Grok adapters

- **Intent:** Each initial driver maps a RunRequest onto its non-interactive CLI form without sharing a giant lowest-common-denominator argv.
- **Expected Outcome:** Codex uses `exec`; Claude uses print mode; Grok uses the local binary's non-interactive entry; unsupported required capabilities surface as such.
- **Validation:** `cargo test -q --test driver_adapt`
- **Status:** Complete

### DRV-003: Unavailable fallback

- **Intent:** A configured fallback list is used only when the chosen driver is unavailable at detect time.
- **Expected Outcome:** Missing default with `fallback = ["codex", "claude"]` selects Claude if Claude is ready; a mid-run failure does not rotate drivers.
- **Validation:** `cargo test -q --test driver_fallback`
- **Status:** Complete

## Execution *(optional)*

Steps: [../execution/DRV.steps.md](../execution/DRV.steps.md)
