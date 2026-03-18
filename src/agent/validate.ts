/**
 * Validation logic for raw TOML agent definitions.
 * Produces clear error messages for invalid or missing fields.
 */

import type { RawAgentToml, ToolDef } from "./schema.js";

export class AgentValidationError extends Error {
  constructor(
    public readonly field: string,
    message: string,
  ) {
    super(`agent config error [${field}]: ${message}`);
    this.name = "AgentValidationError";
  }
}

const VALID_RUNTIMES = new Set(["local", "isolated"]);

/**
 * Validate a raw TOML object and return a list of errors.
 * Returns an empty array if the config is valid.
 */
export function validateRawAgent(raw: RawAgentToml): AgentValidationError[] {
  const errors: AgentValidationError[] = [];

  // system prompt is required
  if (typeof raw.system !== "string" || raw.system.trim() === "") {
    errors.push(
      new AgentValidationError("system", "system prompt is required and must be a non-empty string"),
    );
  }

  // provider — optional but must be a string if present
  if (raw.provider !== undefined && typeof raw.provider !== "string") {
    errors.push(new AgentValidationError("provider", "must be a string"));
  }

  // model — optional but must be a string if present
  if (raw.model !== undefined && typeof raw.model !== "string") {
    errors.push(new AgentValidationError("model", "must be a string"));
  }

  // skills — optional but must be an array of strings
  if (raw.skills !== undefined) {
    if (!Array.isArray(raw.skills)) {
      errors.push(new AgentValidationError("skills", "must be an array of strings"));
    } else if (!raw.skills.every((s) => typeof s === "string")) {
      errors.push(new AgentValidationError("skills", "all entries must be strings"));
    }
  }

  // tools — optional but must be an array of objects with name + description
  if (raw.tools !== undefined) {
    if (!Array.isArray(raw.tools)) {
      errors.push(new AgentValidationError("tools", "must be an array of tool definitions"));
    } else {
      for (let i = 0; i < raw.tools.length; i++) {
        const t = raw.tools[i];
        if (typeof t !== "object" || t === null) {
          errors.push(new AgentValidationError(`tools[${i}]`, "must be an object"));
          continue;
        }
        if (typeof t.name !== "string" || t.name.trim() === "") {
          errors.push(new AgentValidationError(`tools[${i}].name`, "required non-empty string"));
        }
        if (typeof t.description !== "string" || t.description.trim() === "") {
          errors.push(new AgentValidationError(`tools[${i}].description`, "required non-empty string"));
        }
      }
    }
  }

  // runtime — optional but must be a known value
  if (raw.runtime !== undefined) {
    if (typeof raw.runtime !== "string" || !VALID_RUNTIMES.has(raw.runtime)) {
      errors.push(
        new AgentValidationError("runtime", `must be one of: ${[...VALID_RUNTIMES].join(", ")}`),
      );
    }
  }

  // max_iterations — optional, positive integer
  if (raw.max_iterations !== undefined) {
    if (typeof raw.max_iterations !== "number" || !Number.isInteger(raw.max_iterations) || raw.max_iterations < 1) {
      errors.push(new AgentValidationError("max_iterations", "must be a positive integer"));
    }
  }

  // tool_timeout — optional, positive number
  if (raw.tool_timeout !== undefined) {
    if (typeof raw.tool_timeout !== "number" || raw.tool_timeout <= 0) {
      errors.push(new AgentValidationError("tool_timeout", "must be a positive number (ms)"));
    }
  }

  return errors;
}

/** Normalise a validated raw tool entry into a ToolDef */
export function normaliseTools(raw: Array<Record<string, unknown>>): ToolDef[] {
  return raw.map((t) => ({
    name: t.name as string,
    description: t.description as string,
    parameters: (t.parameters as Record<string, unknown>) ?? undefined,
    timeout: typeof t.timeout === "number" ? t.timeout : undefined,
  }));
}
