# ADR 001 — Implement Occam in Rust

| Field | Value |
|-------|-------|
| Status | Accepted |
| Date | 2026-08-24 |
| Supersedes | D-001 (Bun / TypeScript) |

## Context

The 2026-03-15 spec chose Bun for fast startup, single-binary distribution, and TypeScript. That choice assumed Occam would embed provider SDKs and possibly a tool loop.

The v1 rewrite removes those. Occam is a process-spawning Unix utility: clap, TOML, PATH lookup, pipes, timeouts, JSON Schema, a native binary.

## Decision

Implement Occam in Rust. D-001 is void.

Suggested crates are not mandatory: clap, tokio, serde, serde_json, toml, thiserror, tracing, a JSON Schema crate as needed.

## Consequences

- No `package.json`, no Bun runtime, no Node in the product.
- PR #2 (Bun AGENT module) is abandoned.
- Distribution is `cargo install` and GitHub-release binaries.
