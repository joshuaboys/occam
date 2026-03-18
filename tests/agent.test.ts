import { describe, test, expect, beforeAll, afterAll } from "bun:test";
import { mkdtempSync, writeFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { parseAgent, loadAgentFromFile, AgentValidationError, AGENT_DEFAULTS } from "../src/agent/index.js";
import { resolveAgentPath, agentIdFromPath } from "../src/agent/resolve.js";
import { validateRawAgent } from "../src/agent/validate.js";

// --- parseAgent ---

describe("parseAgent", () => {
  test("parses minimal valid agent", () => {
    const toml = `system = "You are a helpful assistant."`;
    const config = parseAgent(toml, "test-agent");

    expect(config.id).toBe("test-agent");
    expect(config.system).toBe("You are a helpful assistant.");
    expect(config.provider).toBe(AGENT_DEFAULTS.provider);
    expect(config.model).toBe(AGENT_DEFAULTS.model);
    expect(config.skills).toEqual([]);
    expect(config.tools).toEqual([]);
    expect(config.runtime).toBe("local");
    expect(config.maxIterations).toBe(10);
    expect(config.toolTimeout).toBe(30_000);
  });

  test("parses fully specified agent", () => {
    const toml = `
id = "reviewer"
system = "You review code."
provider = "openai"
model = "gpt-4o"
skills = ["code-review", "security-audit"]
runtime = "isolated"
max_iterations = 5
tool_timeout = 60000

[[tools]]
name = "read_file"
description = "Read a file from disk"

[[tools]]
name = "grep"
description = "Search file contents"
timeout = 10000
`;
    const config = parseAgent(toml);

    expect(config.id).toBe("reviewer");
    expect(config.provider).toBe("openai");
    expect(config.model).toBe("gpt-4o");
    expect(config.skills).toEqual(["code-review", "security-audit"]);
    expect(config.tools).toHaveLength(2);
    expect(config.tools[0].name).toBe("read_file");
    expect(config.tools[1].timeout).toBe(10000);
    expect(config.runtime).toBe("isolated");
    expect(config.maxIterations).toBe(5);
    expect(config.toolTimeout).toBe(60_000);
  });

  test("uses explicit id over fallback", () => {
    const toml = `id = "custom"\nsystem = "Hello"`;
    const config = parseAgent(toml, "fallback");
    expect(config.id).toBe("custom");
  });

  test("throws on missing system prompt", () => {
    expect(() => parseAgent(`provider = "anthropic"`)).toThrow("system prompt is required");
  });

  test("throws on invalid TOML syntax", () => {
    expect(() => parseAgent("{{invalid")).toThrow("failed to parse agent TOML");
  });

  test("throws on invalid runtime value", () => {
    const toml = `system = "test"\nruntime = "docker"`;
    expect(() => parseAgent(toml)).toThrow("must be one of: local, isolated");
  });

  test("throws on negative max_iterations", () => {
    const toml = `system = "test"\nmax_iterations = -1`;
    expect(() => parseAgent(toml)).toThrow("must be a positive integer");
  });
});

// --- validateRawAgent ---

describe("validateRawAgent", () => {
  test("returns no errors for valid config", () => {
    const errors = validateRawAgent({ system: "Hello" });
    expect(errors).toHaveLength(0);
  });

  test("returns error for empty system", () => {
    const errors = validateRawAgent({ system: "  " });
    expect(errors).toHaveLength(1);
    expect(errors[0].field).toBe("system");
  });

  test("validates tools array entries", () => {
    const errors = validateRawAgent({
      system: "test",
      tools: [{ name: "", description: "desc" }],
    });
    expect(errors.some((e) => e.field.includes("tools[0].name"))).toBe(true);
  });

  test("rejects non-array skills", () => {
    const errors = validateRawAgent({ system: "test", skills: "not-an-array" as any });
    expect(errors.some((e) => e.field === "skills")).toBe(true);
  });
});

// --- resolveAgentPath ---

describe("resolveAgentPath", () => {
  let tmpDir: string;

  beforeAll(() => {
    tmpDir = mkdtempSync(join(tmpdir(), "occam-test-"));
    writeFileSync(join(tmpDir, "myagent.toml"), `system = "test"`);
  });

  afterAll(() => {
    rmSync(tmpDir, { recursive: true, force: true });
  });

  test("resolves direct .toml path", () => {
    const path = resolveAgentPath(join(tmpDir, "myagent.toml"));
    expect(path).toEndWith("myagent.toml");
  });

  test("throws for nonexistent .toml path", () => {
    expect(() => resolveAgentPath("/nonexistent/agent.toml")).toThrow("not found");
  });

  test("throws for nonexistent name", () => {
    expect(() => resolveAgentPath("nonexistent-agent-xyz")).toThrow("not found");
  });
});

// --- agentIdFromPath ---

describe("agentIdFromPath", () => {
  test("extracts id from toml filename", () => {
    expect(agentIdFromPath("/path/to/reviewer.toml")).toBe("reviewer");
  });

  test("extracts id from nested path", () => {
    expect(agentIdFromPath("agents/code-review.toml")).toBe("code-review");
  });
});

// --- loadAgentFromFile ---

describe("loadAgentFromFile", () => {
  let tmpDir: string;

  beforeAll(() => {
    tmpDir = mkdtempSync(join(tmpdir(), "occam-load-"));
    writeFileSync(
      join(tmpDir, "helper.toml"),
      `system = "You help with tasks."\nprovider = "anthropic"\nmodel = "claude-sonnet-4-20250514"`,
    );
  });

  afterAll(() => {
    rmSync(tmpDir, { recursive: true, force: true });
  });

  test("loads and parses a valid file", () => {
    const config = loadAgentFromFile(join(tmpDir, "helper.toml"));
    expect(config.id).toBe("helper");
    expect(config.system).toBe("You help with tasks.");
  });

  test("throws for nonexistent file", () => {
    expect(() => loadAgentFromFile(join(tmpDir, "nope.toml"))).toThrow();
  });
});
