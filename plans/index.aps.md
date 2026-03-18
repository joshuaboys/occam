# Occam v1

| Field   | Value              |
|---------|--------------------|
| Status  | Draft              |
| Owner   | Josh Boys          |
| Created | 2026-03-15         |

## Problem

There is no lightweight, stateless CLI tool for executing single-shot agent tasks with piped Unix-style context. Existing agent frameworks are heavyweight, persistent, and session-oriented — the opposite of the sharp, disposable, composable utility needed for one-off scripted agent runs.

## Success Criteria

- [ ] Single command executes a configured agent run and exits
- [ ] Piped stdin accepted as primary context source
- [ ] Agent definitions resolved from TOML configuration
- [ ] Full runtime context rebuilt deterministically per run
- [ ] Provider abstraction supports at least one LLM backend
- [ ] Optional skill files extend agent instructions at runtime
- [ ] Optional bounded tool use within single execution lifecycle
- [ ] Shell-friendly stdout output suitable for piping
- [ ] Structured output mode available for integration
- [ ] Distributed as a single Bun-built binary
- [ ] No persistent state, memory, or cross-run dependencies

## Constraints

- TypeScript implementation on Bun runtime
- TOML for agent definitions — no YAML, no JSON schema
- Stateless: no run history, no memory, no retained state
- Single-run: execute one task, then terminate — no daemons
- No multi-agent delegation or orchestration
- No full agent-framework SDK as core architecture
- Provider integrations serve Occam's runtime, not the reverse
- Unix composability: stdin/stdout/pipes/scripts must work naturally
- Monitoring is external — no built-in dashboards in v1

## Modules

| Module | Purpose | Status | Dependencies |
|--------|---------|--------|--------------|
| [CLI](./modules/01-cli-entry.aps.md) | Entry point, arg parsing, stdin, invocation | In Progress | — |
| [AGENT](./modules/02-agent-config.aps.md) | TOML agent definition resolution | In Progress | — |
| [CTX](./modules/03-context-assembly.aps.md) | Prompt and runtime context assembly | Draft | AGENT, SKILL |
| [PROVIDER](./modules/04-provider.aps.md) | Provider abstraction and LLM invocation | Draft | CTX |
| [SKILL](./modules/05-skill-support.aps.md) | Reusable skill instruction loading | Draft | — |
| [TOOL](./modules/06-tool-use.aps.md) | Bounded optional tool-use execution | Draft | PROVIDER |
| [OUTPUT](./modules/07-output.aps.md) | Result emission — terminal and structured modes | Draft | — |
| [RUNTIME](./modules/08-runtime.aps.md) | Execution backend selection (local + optional isolated) | Draft | CLI |

## Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Provider SDK coupling leaks into core | High | Strict internal provider contract; SDK behind adapter |
| Tool loops cause unbounded execution | High | Hard iteration cap; timeout per tool invocation |
| TOML config grows into orchestration DSL | Medium | Enforce minimal schema; reject workflow-level constructs |
| Scope creep toward persistent state | High | Design integrity rules enforced at review; no memory APIs |
| Binary size bloats with Bun bundling | Medium | Tree-shake aggressively; audit dependencies per release |

## Open Questions

- Q-001: What is the initial set of supported providers for v1? (At minimum Anthropic; possibly OpenAI)
- Q-002: What tool definitions ship built-in vs user-supplied only?
- Q-003: Exact TOML schema for agent definitions — what fields are required vs optional?
- Q-004: Should the isolated runtime backend (SlicerVM) be included in v1 scope or deferred?

## Decisions

- **D-001:** Bun as runtime — chosen for fast startup, single-binary distribution, and native TypeScript support
- **D-002:** TOML for agent config — chosen for readability and simplicity over YAML/JSON
- **D-003:** No agents SDK as core — Occam owns its execution model; provider SDKs are adapters only
- **D-004:** Stateless by design — each run rebuilds context from scratch; no session continuity
