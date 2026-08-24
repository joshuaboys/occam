# Task Definitions

| ID | Owner | Status |
|----|-------|--------|
| TSK | Josh Boys | Complete |

## Purpose

Named tasks are reusable run semantics: instructions, output mode, limits, and optional capability requirements. A task is not a persona, not an agent, and not a workflow.

## In Scope

- Discover `[task.<name>]` from config/task TOML files
- Fields: `instructions`, `driver`, `stdin`, `output`, `schema`, `cwd`, `timeout`, `max_turns`, `max_output`, `requires`
- Reject unknown keys that look like orchestration (steps, graph, agents, tools, memory)
- Default a missing task name as an error (exit `2`), not as a free-prompt passthrough

## Out of Scope

- Prompt assembly into vendor message formats (→ DRV)
- Execution (→ RUN)
- Skill files as a separate concept
- Task registries, marketplaces, or remote fetch
- Branching, retries, DAGs, or tasks calling tasks

## Interfaces

**Depends on:**

- CFG — file locations and merged tables

**Exposes:**

- Resolved task definition to CLI and RUN

## Ready Checklist

Change status to **Ready** when:

- [x] Purpose and scope are clear
- [x] Dependencies identified
- [x] At least one task defined

## Work Items

### TSK-001: Parse named tasks

- **Intent:** A `[task.review]` table becomes a structured task definition with documented defaults.
- **Expected Outcome:** Required `instructions` present; `output` defaults to `text`; invalid field types fail with exit `2`.
- **Validation:** `cargo test -q --test task_parse`
- **Status:** Complete

### TSK-002: Closed field set

- **Intent:** Task files cannot grow into a workflow DSL.
- **Expected Outcome:** Unknown keys (for example `steps`, `tools`, `agents`) are rejected with a clear error.
- **Validation:** `cargo test -q --test task_closed_schema`
- **Status:** Complete

## Execution *(optional)*

Steps: [../execution/TSK.steps.md](../execution/TSK.steps.md)
