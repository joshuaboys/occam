#!/usr/bin/env bun
/**
 * Occam — stateless single-shot agent runner
 *
 * Entry point. Parses CLI args, reads stdin, runs the lifecycle, and exits.
 */

import { parseArgs, getHelpText, UsageError } from "./cli/args.js";
import { readStdin } from "./cli/stdin.js";
import { run, EXIT_CODES } from "./cli/lifecycle.js";

const VERSION = "0.1.0";

async function main(): Promise<never> {
  let args;
  try {
    args = parseArgs(process.argv.slice(2));
  } catch (err) {
    if (err instanceof UsageError) {
      process.stderr.write(`error: ${err.message}\n\n`);
      process.stderr.write(getHelpText() + "\n");
      process.exit(EXIT_CODES.USAGE_ERROR);
    }
    throw err;
  }

  if (args.help) {
    process.stdout.write(getHelpText() + "\n");
    process.exit(EXIT_CODES.SUCCESS);
  }

  if (args.version) {
    process.stdout.write(`occam ${VERSION}\n`);
    process.exit(EXIT_CODES.SUCCESS);
  }

  // Read piped stdin
  const stdin = await readStdin();

  // Install signal handlers for clean termination
  const cleanup = () => {
    if (args.verbose) {
      process.stderr.write("\n[occam] interrupted\n");
    }
    process.exit(130); // 128 + SIGINT
  };
  process.on("SIGINT", cleanup);
  process.on("SIGTERM", cleanup);

  // Run the lifecycle
  const result = run({
    agentName: args.agent,
    stdin,
    format: args.format,
    verbose: args.verbose,
  });

  if (result.error) {
    process.stderr.write(`error: ${result.error}\n`);
  }

  process.exit(result.exitCode);
}

main();
