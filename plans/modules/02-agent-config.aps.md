# Agent Configuration

| ID | Owner | Status |
|----|-------|--------|
| AGENT | Josh Boys | In Progress |

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

### AGENT-001: TOML agent definition loading and validation

- **Status:** Complete: 2026-03-18
- **Intent:** Parse TOML agent files into validated, normalised AgentConfig structures
- **Expected Outcome:** `loadAgent()` discovers, parses, validates, and normalises agent definitions with sensible defaults
- **Validation:** `bun test tests/agent.test.ts`
- **Files:** `src/agent/schema.ts`, `src/agent/validate.ts`, `src/agent/resolve.ts`, `src/agent/index.ts`

### AGENT-002: Expand TOML schema for advanced features

- **Status:** Draft
- **Intent:** Support additional agent config fields as CTX and PROVIDER modules mature
- **Dependencies:** CTX, PROVIDER modules

## Execution *(optional)*

Steps: [../execution/AGENT.steps.md](../execution/AGENT.steps.md)
