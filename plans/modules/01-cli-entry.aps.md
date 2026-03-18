# CLI Entry Point

| ID | Owner | Status |
|----|-------|--------|
| CLI | Josh Boys | Draft |

## Purpose

Occam's command-line entry point. Parses arguments, reads stdin when piped, resolves the agent to run, and drives the single-run lifecycle from invocation through to clean termination.

## In Scope

- CLI argument parsing and validation
- Stdin detection and reading (piped context)
- Agent name/path resolution from CLI args
- Orchestrating the run lifecycle (resolve → assemble → invoke → emit → exit)
- Exit code semantics (success, failure, usage error)
- Help and version output
- Signal handling for clean termination

## Out of Scope

- Agent definition parsing (→ AGENT)
- Context assembly (→ CTX)
- Provider invocation (→ PROVIDER)
- Output formatting (→ OUTPUT)

## Interfaces

**Depends on:**

- AGENT — resolved agent configuration
- CTX — assembled runtime context
- PROVIDER — LLM invocation
- OUTPUT — result emission

**Exposes:**

- `occam <agent> [options]` — CLI invocation surface
- Parsed invocation inputs (agent name, options, stdin content) to downstream modules

## Ready Checklist

Change status to **Ready** when:

- [ ] Purpose and scope are clear
- [ ] Dependencies identified
- [ ] At least one task defined

## Work Items

*No tasks yet — module is Draft*

## Execution *(optional)*

Steps: [../execution/CLI.steps.md](../execution/CLI.steps.md)
