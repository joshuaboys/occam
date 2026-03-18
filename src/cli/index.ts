/**
 * CLI module — public API
 *
 * Re-exports argument parsing, stdin handling, and lifecycle orchestration.
 */

export { parseArgs, getHelpText, UsageError } from "./args.js";
export type { ParsedArgs } from "./args.js";
export { hasStdin, readStdin } from "./stdin.js";
export { run, EXIT_CODES } from "./lifecycle.js";
export type { RunResult, RunOptions } from "./lifecycle.js";
