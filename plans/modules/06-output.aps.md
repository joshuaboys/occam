# Output and Exit

| ID | Owner | Status |
|----|-------|--------|
| OUT | Josh Boys | Ready |

## Purpose

Makes Occam safe in a pipeline. stdout is the task result. stderr is diagnostics. Structured output is validated or the process fails. Exit codes are Occam's, not the driver's.

## In Scope

- stdout = result; stderr = diagnostics / progress / errors
- `--quiet` suppresses progress, not errors
- Text result vs JSON result body vs `--envelope`
- Schema validation of the result body
- One bounded repair attempt on validation failure
- Refusal to emit invalid structured output
- Exit codes `0`, `2`–`8`
- Driver-native exit retained in stderr and envelope metadata

## Out of Scope

- TUI, dashboards, log files
- Streaming schema output (deferred)
- Scraping JSON out of prose by default
- Monitoring backends

## Interfaces

**Depends on:**

- None — consumes `RunResult`

**Exposes:**

- Bytes on stdout/stderr
- Process exit code

## Ready Checklist

Change status to **Ready** when:

- [x] Purpose and scope are clear
- [x] Dependencies identified
- [x] At least one task defined

## Work Items

### OUT-001: Stream split

- **Intent:** Command substitution captures only the task result.
- **Expected Outcome:** Progress and driver logs never appear on stdout; `--quiet` silences progress on stderr.
- **Validation:** `cargo test -q --test output_streams`

### OUT-002: Schema guarantee

- **Intent:** A schema task either prints valid JSON or fails closed.
- **Expected Outcome:** Valid body → exit `0`; one repair then still invalid → exit `7` and no invalid stdout; unsupported structured output → exit `8`.
- **Validation:** `cargo test -q --test output_schema`

### OUT-003: Exit code mapping

- **Intent:** Callers can branch on Occam codes without knowing the driver.
- **Expected Outcome:** Documented mapping holds for success, usage error, missing driver, auth required, driver fail, timeout, validation fail, unsupported.
- **Validation:** `cargo test -q --test output_exit_codes`

## Execution *(optional)*

Steps: [../execution/OUT.steps.md](../execution/OUT.steps.md)
