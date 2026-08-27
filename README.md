<div align="center">

<img src="assets/occam-logo.png" alt="occam." width="820">

**Stateless Unix-style execution layer for bounded, single-shot agent work.**
**Drives the coding-agent CLIs you already have installed.**

[![CI](https://img.shields.io/github/actions/workflow/status/joshuaboys/occam/ci.yml?label=CI)](https://github.com/joshuaboys/occam/actions)
[![crates.io](https://img.shields.io/crates/v/occam-run)](https://crates.io/crates/occam-run)
[![License](https://img.shields.io/badge/License-MIT-007ec6)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-dea584)](https://www.rust-lang.org)

</div>

---

```bash
git diff main...HEAD | occam review
cat incident.json | occam diagnose --json
OCCAM_DRIVER=claude git diff | occam review
```

A script using Occam should not need to know whether the executor is Codex, Claude Code, or Grok. Not an agent framework. Not a provider SDK. Not a prettier alias of `claude -p`.

Occam is not the intelligence. Occam is the execution contract around locally available intelligence.

## Features

- **Named reusable tasks** — `occam review` instead of a copy-pasted prompt
- **Driver-independent invocation** — the same script runs on Codex, Claude Code, or Grok
- **Clean streams** — stdout is the result, stderr is diagnostics; `result="$(… | occam review)"` is safe
- **Bounded execution** — timeout and max-output enforced by killing the process group
- **Schema-validated JSON** — fail closed; never invalid JSON with exit `0`
- **Stable exit codes** — "not installed" vs "not logged in" vs "timed out", independent of the underlying CLI
- **Predictable stdin** — a pipe is run input; the child never inherits your TTY
- **Single Rust binary** — no daemon, no runtime, no Occam API keys

## Quick start

```bash
# Install
cargo install --git https://github.com/joshuaboys/occam --locked

# Copy examples/occam.toml to ./occam.toml (or ~/.config/occam/config.toml), then:
git diff | occam review
occam drivers    # what this machine can actually run
occam tasks      # named tasks from the resolved config
```

> **Note:** Requires a local `codex`, `claude`, or `grok` CLI. Auth is that CLI's own (`codex login`, `claude /login`, …). Occam never stores those credentials.

## Drivers

| Driver | Non-interactive entry |
|--------|------------------------|
| `codex` | `codex exec` (default driver) |
| `claude` | `claude -p` |
| `grok` | probed from the installed binary (`exec` / `--prompt` / `-p`) |

Selection order:

```text
--driver  →  task.driver  →  OCCAM_DRIVER  →  config default_driver  →  codex
```

Optional unavailable-fallback, detect-time only — not a model router: `fallback = ["codex", "claude"]`

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

Allowed fields: `instructions`, `driver`, `stdin`, `output`, `schema`, `cwd`, `timeout`, `max_turns`, `max_output`, `requires`. Orchestration keys (`steps`, `tools`, `agents`, …) are rejected.

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

<details>
<summary><strong>stdin / stdout / stderr</strong></summary>

- If stdin is a pipe (or redirected file), Occam reads it as run input. If stdin is a TTY, stdin is empty unless `--prompt` supplies text.
- The child does **not** inherit the user's stdin.
- stdout is the task result only. stderr is diagnostics, progress, and errors.
- `--quiet` / `-q` suppresses progress. Errors still go to stderr.

```bash
result="$(git diff | occam review)"
```

</details>

<details>
<summary><strong>Structured output</strong></summary>

- `--json` or `output = "json"`: stdout is the result body.
- `schema` / `--schema`: the body must validate. One bounded repair, then exit `7`. Never invalid JSON with exit `0`.
- `--envelope`: wrap the result in Occam metadata (`version`, `status`, `driver`, `task`, `result`, `duration_ms`).

</details>

<details>
<summary><strong>Configuration</strong></summary>

Discovery, later files override earlier task names: `~/.config/occam/config.toml` and `tasks.toml` → `./occam.toml` → `./.occam.toml`

```toml
default_driver = "codex"
fallback = ["codex", "claude"]
timeout = "120s"

[drivers.codex]
command = "codex"
```

Env: `OCCAM_DRIVER`. No Occam API keys.

</details>

## Product boundary

**Is:** local, single-shot, non-interactive, shell-native, disposable, driver-agnostic.

**Is not:** a daemon, session manager, orchestrator, multi-agent runtime, memory system, MCP platform, auth broker, or a second tool loop. The driver owns reasoning and tools. Occam owns the process boundary.

If Occam ever only saves you from typing slightly different flags, it should be deleted.

## License

MIT — see [LICENSE](LICENSE).
