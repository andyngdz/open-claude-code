# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Users

Developers who prefer Claude Code's harness and want to run it through AI providers other than Anthropic. They use the desktop app while starting a coding session, rather than configuring provider-specific environment variables in every shell.

## Product Purpose

Open Claude Code makes it quick to configure a supported AI provider and launch Claude Code through it. Success means a developer can start a provider-backed Claude Code session without manual proxy setup or shell environment configuration.

## Positioning

A local desktop companion that owns provider configuration and launches Claude Code with the required local gateway settings, replacing per-shell setup used by other routing tools.

## Operating Context

The app runs as a Linux system-tray desktop companion. A developer saves a provider credential, selects a model and workspace, then opens Claude Code in a terminal. The local gateway remains available after the settings window closes.

## Capabilities and Constraints

- The shipped implementation supports OpenCode Go with an API key, model selection, workspace selection, and Claude Code launch.
- Other provider integrations are planned but not implemented.
- The app uses a local gateway and must not add telemetry or send user data to servers it operates. Requests necessarily go to the AI provider selected by the user.
- Provider credentials are stored in the system keyring.

## Brand Commitments

The product name is Open Claude Code. Its value is low-friction provider setup for developers who want to keep using Claude Code.

## Evidence on Hand

- Product intent and current capability are documented in [README.md](README.md).
- The desktop app and local gateway implementation are in `src-tauri/` and `backend/`.
- No customer testimonials, benchmarks, pricing claims, or brand assets beyond the application icons are available in the repository.

## Product Principles

- Keep provider setup out of individual shell sessions.
- Preserve the Claude Code workflow developers already prefer.
- Keep the gateway local and avoid app-operated data collection.
- Make the supported path easy to understand before expanding provider coverage.
