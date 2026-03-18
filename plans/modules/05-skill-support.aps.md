# Skill Support

| ID | Owner | Status |
|----|-------|--------|
| SKILL | Josh Boys | Draft |

## Purpose

Loads optional reusable skill files that refine or extend an agent's execution instructions for the current run. Skills are instruction documents — they influence the prompt, not the runtime behaviour. Skill support is a context-assembly concern, not a route to persistent behavioural state.

## In Scope

- Skill file discovery from agent config references
- Skill file reading and content extraction
- Support for multiple skills per agent run
- Skill content passed to CTX for prompt assembly

## Out of Scope

- Prompt assembly logic (→ CTX)
- Skill authoring tools or generators
- Dynamic skill selection at runtime (skills are declared in agent config)
- Persistent skill state or learning

## Interfaces

**Depends on:**

- None — standalone file loader

**Exposes:**

- Loaded skill content (text) to CTX for assembly

## Ready Checklist

Change status to **Ready** when:

- [ ] Purpose and scope are clear
- [ ] Dependencies identified
- [ ] At least one task defined

## Work Items

*No tasks yet — module is Draft*

## Execution *(optional)*

Steps: [../execution/SKILL.steps.md](../execution/SKILL.steps.md)
