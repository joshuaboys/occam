# Output Model

| ID | Owner | Status |
|----|-------|--------|
| OUTPUT | Josh Boys | Draft |

## Purpose

Emits run results in shell-friendly and structured formats. Output prioritises composability and machine-readability where needed, without compromising straightforward terminal use. Standard output first — structured output as an opt-in mode.

## In Scope

- Plain text output to stdout (default mode)
- Structured output mode (JSON) for integration and observability
- Streaming output during provider invocation
- Clean stderr usage for diagnostics and errors (not mixed with result output)
- Exit code mapping from run outcomes
- Structured lifecycle event emission for external monitoring hooks

## Out of Scope

- Terminal dashboards or TUI (explicitly excluded from v1)
- Colour or formatting libraries beyond basic terminal support
- Log file management
- Monitoring infrastructure

## Interfaces

**Depends on:**

- None — consumes result data from CLI run lifecycle

**Exposes:**

- Output emitter interface to CLI
- Structured event stream for external tools

## Ready Checklist

Change status to **Ready** when:

- [ ] Purpose and scope are clear
- [ ] Dependencies identified
- [ ] At least one task defined

## Work Items

*No tasks yet — module is Draft*

## Execution *(optional)*

Steps: [../execution/OUTPUT.steps.md](../execution/OUTPUT.steps.md)
