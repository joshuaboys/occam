/**
 * AGENT module — public API
 *
 * Resolves, parses, validates, and normalises TOML agent definitions
 * into a typed AgentConfig structure.
 */

import { readFileSync } from "node:fs";
import TOML from "@iarna/toml";
import type { AgentConfig, RawAgentToml } from "./schema.js";
import { AGENT_DEFAULTS } from "./schema.js";
import { validateRawAgent, normaliseTools, AgentValidationError } from "./validate.js";
import { resolveAgentPath, agentIdFromPath } from "./resolve.js";

export type { AgentConfig, ToolDef, RawAgentToml } from "./schema.js";
export { AGENT_DEFAULTS } from "./schema.js";
export { AgentValidationError } from "./validate.js";
export { resolveAgentPath, agentIdFromPath } from "./resolve.js";

/**
 * Load an agent definition from a TOML file path.
 * Parses, validates, and returns a normalised AgentConfig.
 *
 * @param filePath - Absolute or relative path to a .toml agent file
 * @throws Error on file read failure, parse errors, or validation errors
 */
export function loadAgentFromFile(filePath: string): AgentConfig {
  const content = readFileSync(filePath, "utf-8");
  return parseAgent(content, agentIdFromPath(filePath));
}

/**
 * Load an agent by name or path.
 * Resolves the agent file, then loads and validates it.
 *
 * @param nameOrPath - Agent name (searched in standard dirs) or file path
 * @throws Error if agent not found, unparseable, or invalid
 */
export function loadAgent(nameOrPath: string): AgentConfig {
  const filePath = resolveAgentPath(nameOrPath);
  return loadAgentFromFile(filePath);
}

/**
 * Parse a TOML string into a validated AgentConfig.
 *
 * @param tomlContent - Raw TOML string
 * @param fallbackId - ID to use if the TOML doesn't specify one
 */
export function parseAgent(tomlContent: string, fallbackId: string = "unknown"): AgentConfig {
  let raw: RawAgentToml;
  try {
    raw = TOML.parse(tomlContent) as unknown as RawAgentToml;
  } catch (err) {
    throw new Error(`failed to parse agent TOML: ${(err as Error).message}`);
  }

  const errors = validateRawAgent(raw);
  if (errors.length > 0) {
    const messages = errors.map((e) => e.message).join("\n  ");
    throw new Error(`invalid agent definition:\n  ${messages}`);
  }

  return normalise(raw, fallbackId);
}

/** Apply defaults and normalise a validated raw config into AgentConfig */
function normalise(raw: RawAgentToml, fallbackId: string): AgentConfig {
  return {
    id: raw.id ?? fallbackId,
    system: raw.system!,
    provider: raw.provider ?? AGENT_DEFAULTS.provider,
    model: raw.model ?? AGENT_DEFAULTS.model,
    skills: raw.skills ?? [...AGENT_DEFAULTS.skills],
    tools: raw.tools ? normaliseTools(raw.tools) : [...AGENT_DEFAULTS.tools],
    runtime: (raw.runtime as AgentConfig["runtime"]) ?? AGENT_DEFAULTS.runtime,
    maxIterations: raw.max_iterations ?? AGENT_DEFAULTS.maxIterations,
    toolTimeout: raw.tool_timeout ?? AGENT_DEFAULTS.toolTimeout,
  };
}
