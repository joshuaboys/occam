# Tool Use

| ID | Owner | Status |
|----|-------|--------|
| TOOL | Josh Boys | Draft |

## Purpose

Handles bounded, optional tool invocation within a single execution lifecycle. When tools are enabled for an agent, the tool module executes tool calls returned by the provider, feeds results back, and enforces execution limits. Tool support is an execution enhancement — not a licence to turn Occam into an orchestration framework.

## In Scope

- Tool definition loading and registration from agent config
- Tool call extraction from provider responses
- Tool execution dispatch
- Tool result formatting and injection back into provider conversation
- Iteration cap enforcement (max tool rounds per run)
- Per-tool timeout enforcement
- Tool loop lifecycle (call → execute → respond → repeat until done or capped)

## Out of Scope

- Provider communication (→ PROVIDER)
- Nested agent systems or sub-agent delegation (explicitly excluded)
- Multi-agent coordination (explicitly excluded)
- Tool authoring or marketplace

## Interfaces

**Depends on:**

- PROVIDER — tool call responses and continued invocation

**Exposes:**

- Tool execution loop to CLI run lifecycle
- Tool definitions to CTX for prompt injection

## Ready Checklist

Change status to **Ready** when:

- [ ] Purpose and scope are clear
- [ ] Dependencies identified
- [ ] At least one task defined

## Work Items

*No tasks yet — module is Draft*

## Execution *(optional)*

Steps: [../execution/TOOL.steps.md](../execution/TOOL.steps.md)
