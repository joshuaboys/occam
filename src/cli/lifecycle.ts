/**
 * Run lifecycle — orchestrates the single-run execution flow.
 *
 * resolve → assemble → invoke → emit → exit
 *
 * This is the core orchestrator that ties together the AGENT module
 * and future CTX, PROVIDER, and OUTPUT modules.
 */

import type { AgentConfig } from "../agent/index.js";
import { loadAgent } from "../agent/index.js";

/** Exit codes following Unix conventions */
export const EXIT_CODES = {
  SUCCESS: 0,
  GENERAL_ERROR: 1,
  USAGE_ERROR: 2,
  AGENT_NOT_FOUND: 3,
  AGENT_INVALID: 4,
  PROVIDER_ERROR: 5,
} as const;

/** Result of a lifecycle run */
export interface RunResult {
  exitCode: number;
  agent?: AgentConfig;
  stdin?: string | null;
  error?: string;
}

export interface RunOptions {
  agentName: string;
  stdin: string | null;
  format: "text" | "json";
  verbose: boolean;
}

/**
 * Execute the full single-run lifecycle.
 *
 * Currently implements:
 * 1. Agent resolution and loading
 * 2. Stdin capture
 * 3. Placeholder for CTX/PROVIDER/OUTPUT (future modules)
 */
export function run(options: RunOptions): RunResult {
  const { agentName, stdin, format, verbose } = options;

  // Phase 1: Resolve agent
  let agent: AgentConfig;
  try {
    agent = loadAgent(agentName);
  } catch (err) {
    const message = (err as Error).message;
    if (message.includes("not found")) {
      return { exitCode: EXIT_CODES.AGENT_NOT_FOUND, error: message };
    }
    return { exitCode: EXIT_CODES.AGENT_INVALID, error: message };
  }

  if (verbose) {
    process.stderr.write(`[occam] agent: ${agent.id}\n`);
    process.stderr.write(`[occam] provider: ${agent.provider}/${agent.model}\n`);
    if (stdin) {
      process.stderr.write(`[occam] stdin: ${stdin.length} bytes\n`);
    }
    if (agent.skills.length > 0) {
      process.stderr.write(`[occam] skills: ${agent.skills.join(", ")}\n`);
    }
    if (agent.tools.length > 0) {
      process.stderr.write(`[occam] tools: ${agent.tools.map((t) => t.name).join(", ")}\n`);
    }
  }

  // Phase 2: Context assembly (→ CTX module, future)
  // Phase 3: Provider invocation (→ PROVIDER module, future)
  // Phase 4: Output emission (→ OUTPUT module, future)

  // For now, emit a structured summary showing the resolved state
  if (format === "json") {
    const output = {
      agent: agent.id,
      provider: agent.provider,
      model: agent.model,
      system: agent.system,
      skills: agent.skills,
      tools: agent.tools.map((t) => t.name),
      runtime: agent.runtime,
      hasStdin: stdin !== null,
      stdinLength: stdin?.length ?? 0,
      status: "ready",
      message: "Agent resolved. CTX, PROVIDER, and OUTPUT modules pending implementation.",
    };
    process.stdout.write(JSON.stringify(output, null, 2) + "\n");
  } else {
    process.stdout.write(`Agent: ${agent.id}\n`);
    process.stdout.write(`Provider: ${agent.provider}/${agent.model}\n`);
    process.stdout.write(`System: ${agent.system.slice(0, 80)}${agent.system.length > 80 ? "..." : ""}\n`);
    if (stdin) {
      process.stdout.write(`Stdin: ${stdin.length} bytes\n`);
    }
    process.stdout.write(`Status: ready (awaiting CTX/PROVIDER/OUTPUT modules)\n`);
  }

  return { exitCode: EXIT_CODES.SUCCESS, agent, stdin };
}
