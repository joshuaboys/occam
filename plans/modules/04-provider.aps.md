# Provider Abstraction

| ID | Owner | Status |
|----|-------|--------|
| PROVIDER | Josh Boys | Draft |

## Purpose

Defines a normalised internal provider contract for LLM invocation that is independent of any single vendor SDK. Provider integrations exist to execute the Occam runtime, not to define it. SDKs sit behind adapters that conform to Occam's own interface.

## In Scope

- Internal provider interface definition (request/response contract)
- Provider adapter implementation for initial backend(s)
- Provider selection from agent config
- API key / credential resolution from environment
- Streaming and non-streaming invocation modes
- Error normalisation across providers
- Tool-use response parsing (extracting tool calls from provider responses)

## Out of Scope

- Context/prompt assembly (→ CTX)
- Tool execution logic (→ TOOL)
- Output formatting (→ OUTPUT)
- Embedding or fine-tuning APIs
- Multi-provider fan-out or routing

## Interfaces

**Depends on:**

- CTX — assembled runtime context (messages to send)

**Exposes:**

- Provider invocation interface to CLI (run lifecycle)
- Parsed tool-call responses to TOOL module
- Raw completion result to OUTPUT module

## Ready Checklist

Change status to **Ready** when:

- [ ] Purpose and scope are clear
- [ ] Dependencies identified
- [ ] At least one task defined

## Work Items

*No tasks yet — module is Draft*

## Execution *(optional)*

Steps: [../execution/PROVIDER.steps.md](../execution/PROVIDER.steps.md)
