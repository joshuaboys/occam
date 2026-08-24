# Configuration

| ID | Owner | Status |
|----|-------|--------|
| CFG | Josh Boys | Complete |

## Purpose

Resolves a small, predictable configuration: default driver, driver command paths, optional unavailable-fallback, and global limits. Configuration exists so a team script can run on machines with different local CLIs. It is not a routing product.

## In Scope

- Load `~/.config/occam/config.toml`, `./occam.toml`, `./.occam.toml` (later overrides earlier)
- `default_driver`, per-driver `command`, optional `fallback = […]` unavailable-fallback
- `OCCAM_DRIVER` environment override
- Conservative global `timeout` default
- Clear errors for invalid TOML (exit `2`)

## Out of Scope

- Task definition schema (→ TSK)
- Driver capability detection (→ DRV)
- Remote config, registries, or package sources
- API keys, OAuth, or credential files
- Elaborate layered config beyond the three files above

## Interfaces

**Depends on:**

- None

**Exposes:**

- Resolved config object to CLI, TSK, DRV

## Ready Checklist

Change status to **Ready** when:

- [x] Purpose and scope are clear
- [x] Dependencies identified
- [x] At least one task defined

## Work Items

### CFG-001: Load and merge config files

- **Intent:** Occam reads the documented config files with later files overriding earlier keys.
- **Expected Outcome:** Missing files are skipped; invalid TOML fails with exit `2`; defaults apply when unset (`default_driver = "codex"`).
- **Validation:** `cargo test -q --test config_load`
- **Status:** Complete

### CFG-002: Driver command and fallback list

- **Intent:** Each driver has a configurable executable, and an optional fallback list is stored without being interpreted as model routing.
- **Expected Outcome:** `command` overrides PATH name; `fallback = ["codex", "claude"]` is accepted; empty/unknown ids fail validation. (`fallback` rather than `drivers = […]` because TOML cannot combine that array with `[drivers.codex]`.)
- **Validation:** `cargo test -q --test config_drivers`
- **Status:** Complete

## Execution *(optional)*

Steps: [../execution/CFG.steps.md](../execution/CFG.steps.md)
