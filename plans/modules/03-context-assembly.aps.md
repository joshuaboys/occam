# Context Assembly

| ID | Owner | Status |
|----|-------|--------|
| CTX | Josh Boys | Draft |

## Purpose

Reconstructs the full runtime prompt and context for every invocation from the agent definition, skill content, piped input, tool definitions, and selected runtime metadata. The assembled context is deterministic — same inputs always produce the same prompt.

## In Scope

- Assembling the system prompt from agent config
- Merging skill instruction content into the prompt
- Incorporating piped stdin as user-supplied context
- Injecting tool definitions when tools are enabled
- Including selected runtime metadata (e.g. working directory, timestamp)
- Producing a final provider-ready message structure

## Out of Scope

- Agent TOML parsing (→ AGENT)
- Skill file discovery and loading (→ SKILL)
- Persistent memory or conversation history (explicitly excluded)
- Provider-specific message formatting (→ PROVIDER)

## Interfaces

**Depends on:**

- AGENT — resolved agent configuration
- SKILL — loaded skill content

**Exposes:**

- Assembled runtime context (messages/prompt structure) to PROVIDER

## Ready Checklist

Change status to **Ready** when:

- [ ] Purpose and scope are clear
- [ ] Dependencies identified
- [ ] At least one task defined

## Work Items

*No tasks yet — module is Draft*

## Execution *(optional)*

Steps: [../execution/CTX.steps.md](../execution/CTX.steps.md)
