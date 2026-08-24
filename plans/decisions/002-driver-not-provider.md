# ADR 002 — Driver over local CLI, not provider SDK

| Field | Value |
|-------|-------|
| Status | Accepted |
| Date | 2026-08-24 |
| Supersedes | PROVIDER module, Q-001 |

## Context

The previous spec treated Occam as a client of LLM vendors: API keys, adapter SDKs, an internal provider contract. That duplicates Codex, Claude Code, and Grok, and it makes Occam responsible for auth it should not own.

The product test is “why not call `claude -p`?”. Owning a provider SDK fails that test: the user already has a logged-in CLI.

## Decision

The unit of integration is a **driver**: a locally installed executable. Occam detects it, invokes it non-interactively, and normalises the result. Occam does not call model HTTP APIs, store API keys, or run OAuth.

Initial drivers: Codex (default), Claude Code, Grok.

## Consequences

- No Occam login command in v1.
- Capability gaps are exit `8`, not a fake lowest-common-denominator.
- Account switching, if wanted, is the user's CLI or an external wrapper (`command = …`). Occam does not become an auth broker.
