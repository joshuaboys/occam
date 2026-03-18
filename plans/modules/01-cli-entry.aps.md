# CLI Entry Point

| ID | Owner | Status |
|----|-------|--------|
| CLI | Josh Boys | In Progress |

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

### CLI-001: CLI argument parsing and stdin reading

- **Status:** Complete: 2026-03-18
- **Intent:** Parse CLI arguments and read piped stdin into typed structures
- **Expected Outcome:** `parseArgs()` handles all flags, `readStdin()` captures piped input
- **Validation:** `bun test tests/cli.test.ts`
- **Files:** `src/cli/args.ts`, `src/cli/stdin.ts`

### CLI-002: Lifecycle orchestration and entry point

- **Status:** Complete: 2026-03-18
- **Intent:** Orchestrate the single-run lifecycle from invocation to exit
- **Expected Outcome:** `run()` resolves agent, captures stdin, emits output, returns exit code
- **Validation:** `bun test tests/cli.test.ts`
- **Files:** `src/cli/lifecycle.ts`, `src/main.ts`

### CLI-003: Wire CTX, PROVIDER, and OUTPUT modules into lifecycle

- **Status:** Draft
- **Intent:** Replace placeholder output with real context assembly, provider invocation, and output emission
- **Expected Outcome:** Full end-to-end agent execution through the CLI
- **Dependencies:** CTX, PROVIDER, OUTPUT modules

## Execution *(optional)*

Steps: [../execution/CLI.steps.md](../execution/CLI.steps.md)
