# Runtime Backend

| ID | Owner | Status |
|----|-------|--------|
| RUNTIME | Josh Boys | Draft |

## Purpose

Manages execution backend selection between local execution (default) and optional isolated disposable environments. The same agent model and run contract apply regardless of runtime target. Runtime selection is an execution backend concern — not a change to Occam's core identity.

## In Scope

- Runtime backend abstraction (local vs isolated)
- Local runtime implementation (default — direct execution on host)
- Optional isolated runtime integration (SlicerVM for licensed users)
- Runtime selection from agent config or CLI flags
- Environment setup and teardown for isolated backends
- Ensuring stateless single-run lifecycle in all backends

## Out of Scope

- VM platform management (Occam consumes pre-existing configs, not manages VMs)
- Isolated backend becoming required for standard usage
- Long-lived daemon or background operation
- Runtime-specific agent behaviour changes

## Interfaces

**Depends on:**

- CLI — runtime selection from invocation

**Exposes:**

- Runtime execution interface to CLI run lifecycle

## Ready Checklist

Change status to **Ready** when:

- [ ] Purpose and scope are clear
- [ ] Dependencies identified
- [ ] At least one task defined

## Work Items

*No tasks yet — module is Draft*

## Execution *(optional)*

Steps: [../execution/RUNTIME.steps.md](../execution/RUNTIME.steps.md)
