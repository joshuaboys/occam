# Agent Configuration

| ID | Owner | Status |
|----|-------|--------|
| AGENT | Josh Boys | Draft |

## Purpose

Resolves and parses TOML agent definitions into a normalised internal configuration structure. Each agent definition describes one execution profile for one class of task — not a persona or persistent identity.

## In Scope

- TOML agent definition file discovery and loading
- TOML parsing and validation against expected schema
- Normalised agent config structure (system prompt, provider, model, skills, tools)
- Default resolution for optional fields
- Error reporting for invalid or missing agent definitions

## Out of Scope

- Agent file format negotiation (TOML only — no YAML, JSON, etc.)
- Prompt assembly from agent config (→ CTX)
- Skill file loading (→ SKILL)
- Orchestration or workflow semantics

## Interfaces

**Depends on:**

- None — standalone module

**Exposes:**

- Resolved agent configuration object to CTX and CLI

## Ready Checklist

Change status to **Ready** when:

- [ ] Purpose and scope are clear
- [ ] Dependencies identified
- [ ] At least one task defined

## Work Items

*No tasks yet — module is Draft*

## Execution *(optional)*

Steps: [../execution/AGENT.steps.md](../execution/AGENT.steps.md)
