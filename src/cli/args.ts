/**
 * CLI argument parsing and validation.
 * Produces a typed ParsedArgs structure from process.argv.
 */

/** Parsed CLI invocation */
export interface ParsedArgs {
  /** Agent name or path (positional argument) */
  agent: string;
  /** Output format */
  format: "text" | "json";
  /** Show help */
  help: boolean;
  /** Show version */
  version: boolean;
  /** Enable verbose/debug output to stderr */
  verbose: boolean;
  /** Additional pass-through arguments */
  rest: string[];
}

export class UsageError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "UsageError";
  }
}

const HELP_TEXT = `
occam — stateless single-shot agent runner

USAGE
  occam <agent> [options]
  echo "context" | occam <agent>

ARGUMENTS
  <agent>    Agent name (searches ./agents/, ./.occam/) or path to .toml file

OPTIONS
  --format <text|json>   Output format (default: text)
  --verbose              Enable diagnostic output on stderr
  --help, -h             Show this help message
  --version, -v          Show version

EXAMPLES
  occam reviewer                    Run the "reviewer" agent
  cat file.ts | occam code-review   Pipe file content as context
  occam ./my-agent.toml --format json
`.trim();

export function getHelpText(): string {
  return HELP_TEXT;
}

/**
 * Parse raw argv (typically process.argv.slice(2)) into ParsedArgs.
 * Throws UsageError for invalid combinations.
 */
export function parseArgs(argv: string[]): ParsedArgs {
  const result: ParsedArgs = {
    agent: "",
    format: "text",
    help: false,
    version: false,
    verbose: false,
    rest: [],
  };

  let i = 0;
  while (i < argv.length) {
    const arg = argv[i];

    if (arg === "--help" || arg === "-h") {
      result.help = true;
      i++;
    } else if (arg === "--version" || arg === "-v") {
      result.version = true;
      i++;
    } else if (arg === "--verbose") {
      result.verbose = true;
      i++;
    } else if (arg === "--format") {
      i++;
      const val = argv[i];
      if (val !== "text" && val !== "json") {
        throw new UsageError(`invalid format "${val}" — must be "text" or "json"`);
      }
      result.format = val;
      i++;
    } else if (arg.startsWith("--")) {
      throw new UsageError(`unknown option: ${arg}`);
    } else if (result.agent === "") {
      result.agent = arg;
      i++;
    } else {
      result.rest.push(arg);
      i++;
    }
  }

  // Validate: agent is required unless help/version
  if (!result.help && !result.version && result.agent === "") {
    throw new UsageError("missing required argument: <agent>");
  }

  return result;
}
