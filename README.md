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

## Install

```bash
cargo install --git https://github.com/joshuaboys/occam --locked
```

Requires a local `codex`, `claude`, or `grok` CLI. Auth is that CLI's (`codex login`, `claude /login`, …). Occam never stores those credentials.

## Quick start

Copy [examples/occam.toml](examples/occam.toml) to `./occam.toml` (or `~/.config/occam/config.toml`), then:

```bash
git diff | occam review
occam drivers    # what this machine can actually run
occam tasks      # named tasks from the resolved config
```

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

## Drivers

| Driver | Default command | Non-interactive entry |
|--------|-----------------|------------------------|
| `codex` | `codex` | `codex exec` (default) |
| `claude` | `claude` | `claude -p` |
| `grok` | `grok` | probed from the installed binary (`exec` / `--prompt` / `-p`) |

```text
--driver  →  task.driver  →  OCCAM_DRIVER  →  config default_driver  →  codex
```

Optional unavailable-fallback, detect-time only — not a model router:

```toml
fallback = ["codex", "claude"]
```

Named `fallback` because TOML cannot use `drivers = [...]` alongside `[drivers.codex]`.

## Tasks

A named task is reusable run semantics. It is not a persona, not a workflow, not an agent.

```toml
[task.review]
instructions = """
Review the supplied change.
Report only material correctness, security, or maintainability issues.
"""
output = "text"
timeout = "90s"
stdin = "required"
```

Allowed fields: `instructions`, `driver`, `stdin` (`required` \| `optional` \| `forbidden`), `output` (`text` \| `json`), `schema`, `cwd`, `timeout`, `max_turns`, `max_output`, `requires`.

Orchestration keys (`steps`, `tools`, `agents`, `memory`, `mcp`, …) are rejected.

Discovery, later files override earlier task names:

1. `~/.config/occam/config.toml` and `~/.config/occam/tasks.toml`
2. `./occam.toml`
3. `./.occam.toml`

## stdin / stdout / stderr

- If stdin is a pipe (or redirected file), Occam reads it as run input.
- If stdin is a TTY, stdin is empty unless `--prompt` supplies text.
- The child does **not** inherit the user's stdin.
- stdout is the task result only. stderr is diagnostics, progress, and errors.
- `--quiet` / `-q` suppresses progress. Errors still go to stderr.

```bash
result="$(git diff | occam review)"
```

## Structured output

- `--json` or `output = "json"`: stdout is the result body.
- `schema` / `--schema`: the body must validate. One bounded repair, then exit `7`. Never invalid JSON with exit `0`.
- `--envelope`: wrap the result in Occam metadata (`version`, `status`, `driver`, `task`, `result`, `duration_ms`).

## Bounds

| Limit | Enforcement |
|-------|-------------|
| `timeout` | Occam kills the process group. Default `120s` if unset. |
| `max_output` | Occam stops reading and kills the process group. |
| `max_turns` | Translated when the driver advertises it; otherwise exit `8`. |

Cancel (SIGINT/SIGTERM) tears down the child's process group.

## Exit codes

| Code | Meaning |
|------|---------|
| `0` | completed successfully |
| `2` | invalid invocation or configuration |
| `3` | driver unavailable (not on `PATH`) |
| `4` | driver authentication or setup required |
| `5` | driver execution failed |
| `6` | timeout |
| `7` | structured output validation failed |
| `8` | unsupported requested capability |

Driver-native exit codes stay on stderr and in envelope metadata. They never leak as Occam's process exit except via the mapping above.

## CLI

```text
occam <task> [--driver NAME] [--json] [--envelope] [--schema PATH]
             [--cwd PATH] [--timeout DURATION] [--prompt TEXT] [-q]
occam drivers
occam tasks
```

## Config

```toml
default_driver = "codex"
fallback = ["codex", "claude"]
timeout = "120s"

[drivers.codex]
command = "codex"
```

Env: `OCCAM_DRIVER`. No Occam API keys.

## Language

**Rust.** The 2026-03 Bun decision is superseded. See [plans/decisions/001-rust.md](plans/decisions/001-rust.md).

Canonical design: [designs/2026-08-24-occam-v1.design.md](designs/2026-08-24-occam-v1.design.md)

## License

MIT. See [LICENSE](LICENSE).
