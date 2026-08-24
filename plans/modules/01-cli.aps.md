# CLI Entry

| ID | Owner | Status |
|----|-------|--------|
| CLI | Josh Boys | Complete |

## Purpose

Occam's command-line surface. Parses arguments, decides whether stdin is a pipe, resolves the named task, and exits with Occam's stable codes. Feels like a Unix utility: `git diff | occam review`.

## In Scope

- `occam <task> [options]`
- `occam drivers`, `occam tasks`, `--help`, `--version`
- Flags: `--driver`, `--json`, `--envelope`, `--schema`, `--cwd`, `--timeout`, `--prompt`, `--quiet`
- Piped-stdin detection (pipe vs TTY)
- Signal handling so cancel tears down the child
- Mapping run outcomes onto Occam exit codes

## Out of Scope

- Task file parsing (→ TSK)
- Config file parsing (→ CFG)
- Driver adapters (→ DRV)
- Process spawn details (→ RUN)
- Schema validation (→ OUT)

## Interfaces

**Depends on:**

- CFG — resolved configuration
- TSK — named task
- DRV — selected driver
- RUN — child-process lifecycle
- OUT — result emission and exit mapping

**Exposes:**

- `occam <task>` invocation
- `occam drivers` / `occam tasks` inspection commands

## Ready Checklist

Change status to **Ready** when:

- [x] Purpose and scope are clear
- [x] Dependencies identified
- [x] At least one task defined

## Work Items

### CLI-001: Task invocation surface

- **Intent:** A user can run a named task as `occam <task>` with the documented flags.
- **Expected Outcome:** `--help` lists the invocation; unknown flags and missing tasks fail with exit `2`.
- **Validation:** `cargo test -q --test cli_invoke && cargo run -- --help`
- **Status:** Complete

### CLI-002: Inspection commands

- **Intent:** Operators can see which drivers and tasks this machine knows without calling a model.
- **Expected Outcome:** `occam drivers` and `occam tasks` print to stdout and exit `0` when config is valid.
- **Validation:** `cargo test -q --test cli_inspect`
- **Status:** Complete

### CLI-003: Stdin detection

- **Intent:** Piped input is task context; a TTY is not silently treated as a pipe.
- **Expected Outcome:** A pipe is read; a TTY yields empty stdin unless `--prompt` is set; required-stdin tasks fail with exit `2` when empty.
- **Validation:** `cargo test -q --test cli_stdin`
- **Status:** Complete

## Execution *(optional)*

Steps: [../execution/CLI.steps.md](../execution/CLI.steps.md)
