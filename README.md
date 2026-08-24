# Occam

**Stateless Unix-style execution layer for bounded, single-shot agent work.**

Occam drives coding-agent CLIs you already have installed and authenticated. It is not an agent framework, not a provider SDK, and not a prettier alias of `claude -p` or `codex exec`.

```bash
git diff main...HEAD | occam review
cat incident.json | occam diagnose --json
OCCAM_DRIVER=claude git diff | occam review
```

A script using Occam should not need to know whether the executor is Codex, Claude Code, or Grok.

```text
stdin / args / task  →  bounded run  →  local driver (codex | claude | grok)  →  stdout / JSON / exit
```

Occam is not the intelligence. Occam is the execution contract around locally available intelligence.

## Status

Spec rewrite, August 2026. Implementation has not started. The previous Bun/TypeScript + PROVIDER plan is **void**.

Canonical design: [designs/2026-08-24-occam-v1.design.md](designs/2026-08-24-occam-v1.design.md)

Plan: [plans/index.aps.md](plans/index.aps.md)

## Why this exists

Direct CLI invocations are not a portable contract. Each coding-agent CLI has different flags, stdin behaviour, JSON, timeouts, and exit codes. Occam supplies:

- reusable named tasks
- predictable stdin and cwd
- bounded execution (timeout, process teardown)
- driver-independent invocation
- stdout = result, stderr = diagnostics
- optional schema-validated JSON (fail closed)
- stable exit codes
- portability across machines that have different CLIs installed

If Occam ever only saves you from typing slightly different flags, it should be deleted.

## Product boundary

**Is:** local, single-shot, non-interactive, shell-native, disposable, driver-agnostic.

**Is not:** a daemon, session manager, orchestrator, multi-agent runtime, memory system, MCP platform, auth broker, or a second tool loop.

The driver owns reasoning and tools. Occam owns the process boundary.

Auth is the underlying CLI's (`codex`, `claude`, `grok`). Occam never stores those credentials.

## v1 drivers

| Driver | Default command | Role |
|--------|-----------------|------|
| `codex` | `codex` | Default |
| `claude` | `claude` | Claude Code |
| `grok` | `grok` | xAI Grok CLI |

```text
--driver  →  task.driver  →  OCCAM_DRIVER  →  config  →  codex
```

## Language

**Rust.** The 2026-03 Bun decision is superseded. See [plans/decisions/001-rust.md](plans/decisions/001-rust.md).

## License

MIT. See [LICENSE](LICENSE).
