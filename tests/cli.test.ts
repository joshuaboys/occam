import { describe, test, expect, beforeAll, afterAll } from "bun:test";
import { mkdtempSync, writeFileSync, mkdirSync, rmSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { parseArgs, getHelpText, UsageError } from "../src/cli/args.js";
import { run, EXIT_CODES } from "../src/cli/lifecycle.js";

// --- parseArgs ---

describe("parseArgs", () => {
  test("parses agent name", () => {
    const args = parseArgs(["reviewer"]);
    expect(args.agent).toBe("reviewer");
    expect(args.format).toBe("text");
    expect(args.help).toBe(false);
    expect(args.version).toBe(false);
    expect(args.verbose).toBe(false);
  });

  test("parses --format json", () => {
    const args = parseArgs(["agent", "--format", "json"]);
    expect(args.format).toBe("json");
  });

  test("parses --verbose", () => {
    const args = parseArgs(["agent", "--verbose"]);
    expect(args.verbose).toBe(true);
  });

  test("parses --help", () => {
    const args = parseArgs(["--help"]);
    expect(args.help).toBe(true);
    expect(args.agent).toBe(""); // no agent needed with --help
  });

  test("parses -h shorthand", () => {
    const args = parseArgs(["-h"]);
    expect(args.help).toBe(true);
  });

  test("parses --version", () => {
    const args = parseArgs(["--version"]);
    expect(args.version).toBe(true);
  });

  test("parses -v shorthand", () => {
    const args = parseArgs(["-v"]);
    expect(args.version).toBe(true);
  });

  test("throws UsageError for missing agent", () => {
    expect(() => parseArgs([])).toThrow(UsageError);
    expect(() => parseArgs([])).toThrow("missing required argument");
  });

  test("throws UsageError for invalid format", () => {
    expect(() => parseArgs(["agent", "--format", "xml"])).toThrow("invalid format");
  });

  test("throws UsageError for unknown option", () => {
    expect(() => parseArgs(["agent", "--unknown"])).toThrow("unknown option");
  });

  test("collects rest arguments", () => {
    const args = parseArgs(["agent", "extra1", "extra2"]);
    expect(args.rest).toEqual(["extra1", "extra2"]);
  });

  test("parses agent with .toml path", () => {
    const args = parseArgs(["./agents/custom.toml"]);
    expect(args.agent).toBe("./agents/custom.toml");
  });

  test("handles all options combined", () => {
    const args = parseArgs(["myagent", "--format", "json", "--verbose"]);
    expect(args.agent).toBe("myagent");
    expect(args.format).toBe("json");
    expect(args.verbose).toBe(true);
  });
});

// --- getHelpText ---

describe("getHelpText", () => {
  test("contains usage info", () => {
    const help = getHelpText();
    expect(help).toContain("occam <agent>");
    expect(help).toContain("--format");
    expect(help).toContain("--help");
    expect(help).toContain("EXAMPLES");
  });
});

// --- EXIT_CODES ---

describe("EXIT_CODES", () => {
  test("defines expected codes", () => {
    expect(EXIT_CODES.SUCCESS).toBe(0);
    expect(EXIT_CODES.GENERAL_ERROR).toBe(1);
    expect(EXIT_CODES.USAGE_ERROR).toBe(2);
    expect(EXIT_CODES.AGENT_NOT_FOUND).toBe(3);
    expect(EXIT_CODES.AGENT_INVALID).toBe(4);
    expect(EXIT_CODES.PROVIDER_ERROR).toBe(5);
  });
});

// --- run (lifecycle) ---

describe("run", () => {
  let tmpDir: string;

  beforeAll(() => {
    tmpDir = mkdtempSync(join(tmpdir(), "occam-cli-"));
    writeFileSync(
      join(tmpDir, "test-agent.toml"),
      `system = "You are a test agent."\nprovider = "anthropic"\nmodel = "claude-sonnet-4-20250514"`,
    );
  });

  afterAll(() => {
    rmSync(tmpDir, { recursive: true, force: true });
  });

  test("runs successfully with valid agent path", () => {
    const result = run({
      agentName: join(tmpDir, "test-agent.toml"),
      stdin: null,
      format: "text",
      verbose: false,
    });
    expect(result.exitCode).toBe(EXIT_CODES.SUCCESS);
    expect(result.agent).toBeDefined();
    expect(result.agent!.id).toBe("test-agent");
  });

  test("returns AGENT_NOT_FOUND for missing agent", () => {
    const result = run({
      agentName: join(tmpDir, "nonexistent.toml"),
      stdin: null,
      format: "text",
      verbose: false,
    });
    expect(result.exitCode).toBe(EXIT_CODES.AGENT_NOT_FOUND);
    expect(result.error).toContain("not found");
  });

  test("passes stdin through", () => {
    const result = run({
      agentName: join(tmpDir, "test-agent.toml"),
      stdin: "some piped content",
      format: "text",
      verbose: false,
    });
    expect(result.exitCode).toBe(EXIT_CODES.SUCCESS);
    expect(result.stdin).toBe("some piped content");
  });

  test("json format produces valid output", () => {
    const result = run({
      agentName: join(tmpDir, "test-agent.toml"),
      stdin: null,
      format: "json",
      verbose: false,
    });
    expect(result.exitCode).toBe(EXIT_CODES.SUCCESS);
  });
});

// --- Integration: main entry via subprocess ---

describe("main entry point", () => {
  let tmpDir: string;

  beforeAll(() => {
    tmpDir = mkdtempSync(join(tmpdir(), "occam-main-"));
    writeFileSync(
      join(tmpDir, "hello.toml"),
      `system = "Say hello."`,
    );
  });

  afterAll(() => {
    rmSync(tmpDir, { recursive: true, force: true });
  });

  test("--help exits 0 with usage text", async () => {
    const proc = Bun.spawn(["bun", "run", "src/main.ts", "--help"], {
      cwd: "/home/user/occam",
      stdout: "pipe",
      stderr: "pipe",
    });
    const exitCode = await proc.exited;
    const stdout = await new Response(proc.stdout).text();
    expect(exitCode).toBe(0);
    expect(stdout).toContain("occam <agent>");
  });

  test("--version exits 0 with version", async () => {
    const proc = Bun.spawn(["bun", "run", "src/main.ts", "--version"], {
      cwd: "/home/user/occam",
      stdout: "pipe",
      stderr: "pipe",
    });
    const exitCode = await proc.exited;
    const stdout = await new Response(proc.stdout).text();
    expect(exitCode).toBe(0);
    expect(stdout).toContain("occam 0.1.0");
  });

  test("no args exits 2 with usage error", async () => {
    const proc = Bun.spawn(["bun", "run", "src/main.ts"], {
      cwd: "/home/user/occam",
      stdout: "pipe",
      stderr: "pipe",
    });
    const exitCode = await proc.exited;
    const stderr = await new Response(proc.stderr).text();
    expect(exitCode).toBe(2);
    expect(stderr).toContain("missing required argument");
  });

  test("valid agent resolves and runs", async () => {
    const proc = Bun.spawn(
      ["bun", "run", "src/main.ts", join(tmpDir, "hello.toml")],
      {
        cwd: "/home/user/occam",
        stdout: "pipe",
        stderr: "pipe",
      },
    );
    const exitCode = await proc.exited;
    const stdout = await new Response(proc.stdout).text();
    expect(exitCode).toBe(0);
    expect(stdout).toContain("Agent: hello");
  });

  test("piped stdin is captured", async () => {
    // Use a shell pipe to ensure stdin.isTTY is false
    const proc = Bun.spawn(
      ["bash", "-c", `echo -n "hello world" | bun run src/main.ts ${join(tmpDir, "hello.toml")} --format json`],
      {
        cwd: "/home/user/occam",
        stdout: "pipe",
        stderr: "pipe",
      },
    );
    const exitCode = await proc.exited;
    const stdout = await new Response(proc.stdout).text();
    expect(exitCode).toBe(0);
    const output = JSON.parse(stdout);
    expect(output.hasStdin).toBe(true);
    expect(output.stdinLength).toBe(11);
  });

  test("nonexistent agent exits 3", async () => {
    const proc = Bun.spawn(
      ["bun", "run", "src/main.ts", join(tmpDir, "nope.toml")],
      {
        cwd: "/home/user/occam",
        stdout: "pipe",
        stderr: "pipe",
      },
    );
    const exitCode = await proc.exited;
    expect(exitCode).toBe(3);
  });
});
