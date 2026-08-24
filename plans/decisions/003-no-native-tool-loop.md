# ADR 003 — No Occam-native tool loop

| Field | Value |
|-------|-------|
| Status | Accepted |
| Date | 2026-08-24 |
| Supersedes | TOOL module, SKILL module, Q-002 |

## Context

Codex, Claude Code, and Grok already own filesystem tools, shell, search, MCP, web, and repo edits. An Occam tool loop would compete with them, expand scope, and reintroduce unbounded execution — the thing Occam is supposed to bound from the outside.

No v1 use case requires a tool Occam can offer that the driver cannot.

## Decision

Occam does not implement a generic tool loop, skill loader, or MCP host.

> Occam controls execution boundaries. The driver controls agent-native reasoning and tools.

Skills collapse into task `instructions`. If a future use case cannot be delegated to drivers, revisit this ADR with that use case in writing.

## Consequences

- TOOL and SKILL modules are deleted.
- `max_turns` is a translated driver flag, not an Occam-side tool counter.
- Sandbox and tool allowlists are driver capabilities, not Occam runtimes.
