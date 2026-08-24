# Run Lifecycle

| ID | Owner | Status |
|----|-------|--------|
| RUN | Josh Boys | Ready |

## Purpose

A run is one child-process lifecycle. Occam launches the selected driver, controls stdin, sets cwd, enforces external limits, tears the process down, and returns a result. No run state survives unless emitted to the caller.

## In Scope

- Spawn the driver command in a new process group
- Controlled stdin (never inherit the user's TTY by accident)
- Well-defined cwd (`--cwd` or invoke directory)
- External timeout (always)
- External max-output-bytes (always)
- Clean terminate on timeout or cancel (SIGTERM then SIGKILL)
- Distinguish external limits from translated limits from unsupported limits

## Out of Scope

- Driver argv mapping (→ DRV)
- Schema validation and repair (→ OUT)
- Isolated VMs or extra sandboxes beyond the driver's own flags
- Persistent sessions, retries across processes, or daemons
- Occam-native tool execution

## Interfaces

**Depends on:**

- DRV — mapped command, detect status, advertised capabilities
- TSK — instructions, limits, capability requirements

**Exposes:**

- `RunResult` (status, bytes, duration, driver exit) to OUT

## Ready Checklist

Change status to **Ready** when:

- [x] Purpose and scope are clear
- [x] Dependencies identified
- [x] At least one task defined

## Work Items

### RUN-001: Spawn with controlled stdin and cwd

- **Intent:** The child receives Occam-supplied input and a defined cwd, not an inherited TTY.
- **Expected Outcome:** Piped bytes reach the driver as input; the child's working directory is the resolved cwd.
- **Validation:** `cargo test -q --test run_spawn`

### RUN-002: External timeout and teardown

- **Intent:** A stuck driver cannot hang a pipeline, and children do not outlive Occam.
- **Expected Outcome:** Exceeded timeout yields exit `6`; the process group is gone; cancel takes the child with it.
- **Validation:** `cargo test -q --test run_timeout`

## Execution *(optional)*

Steps: [../execution/RUN.steps.md](../execution/RUN.steps.md)
