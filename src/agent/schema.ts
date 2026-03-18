/**
 * Agent configuration schema — the normalised internal structure
 * produced after parsing and validating a TOML agent definition.
 */

/** Tool definition within an agent config */
export interface ToolDef {
  name: string;
  description: string;
  parameters?: Record<string, unknown>;
  timeout?: number;
}

/** Fully resolved agent configuration */
export interface AgentConfig {
  /** Agent identifier (derived from filename or explicit id field) */
  id: string;
  /** System prompt — the core instruction for this agent */
  system: string;
  /** LLM provider name (e.g. "anthropic", "openai") */
  provider: string;
  /** Model identifier (e.g. "claude-sonnet-4-20250514") */
  model: string;
  /** Optional skill file paths to merge into the prompt */
  skills: string[];
  /** Optional tool definitions available to this agent */
  tools: ToolDef[];
  /** Runtime backend — "local" or "isolated" */
  runtime: "local" | "isolated";
  /** Maximum tool-use iterations per run */
  maxIterations: number;
  /** Per-tool timeout in milliseconds */
  toolTimeout: number;
}

/** Raw TOML structure before normalisation */
export interface RawAgentToml {
  id?: string;
  system?: string;
  provider?: string;
  model?: string;
  skills?: string[];
  tools?: Array<Record<string, unknown>>;
  runtime?: string;
  max_iterations?: number;
  tool_timeout?: number;
  [key: string]: unknown;
}

/** Defaults applied when optional fields are absent */
export const AGENT_DEFAULTS = {
  provider: "anthropic",
  model: "claude-sonnet-4-20250514",
  skills: [] as string[],
  tools: [] as ToolDef[],
  runtime: "local" as const,
  maxIterations: 10,
  toolTimeout: 30_000,
} as const;
