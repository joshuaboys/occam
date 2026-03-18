/**
 * Agent definition discovery and resolution.
 * Finds TOML agent files by name or path.
 */

import { existsSync } from "node:fs";
import { resolve, basename, extname } from "node:path";

/** Standard locations to search for agent definitions, in priority order */
const SEARCH_PATHS = [
  ".", // current directory
  "./agents", // agents/ subdirectory
  "./.occam", // hidden config directory
];

/**
 * Resolve an agent name or path to an absolute TOML file path.
 *
 * Resolution order:
 * 1. If input is an existing file path (absolute or relative), use it directly
 * 2. If input is a name (no extension), search standard locations for `<name>.toml`
 *
 * @throws Error if agent definition cannot be found
 */
export function resolveAgentPath(input: string): string {
  // Direct path — absolute or relative with extension
  if (input.endsWith(".toml")) {
    const abs = resolve(input);
    if (existsSync(abs)) return abs;
    throw new Error(`agent definition not found: ${input}`);
  }

  // Name-based search
  const filename = `${input}.toml`;
  for (const dir of SEARCH_PATHS) {
    const candidate = resolve(dir, filename);
    if (existsSync(candidate)) return candidate;
  }

  throw new Error(
    `agent "${input}" not found. Searched: ${SEARCH_PATHS.map((d) => `${d}/${filename}`).join(", ")}`,
  );
}

/** Extract an agent ID from a file path (filename without extension) */
export function agentIdFromPath(filePath: string): string {
  return basename(filePath, extname(filePath));
}
