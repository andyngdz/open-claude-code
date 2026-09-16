# Open Claude Code

Launch Claude Code through the AI provider you choose.

Open Claude Code is a planned local proxy and desktop companion. It will keep
provider setup in a system tray app, then open a terminal with the selected
provider and proxy configuration ready for a new Claude Code session.

## Planned experience

- Open the dashboard from the system tray.
- Configure OpenCode, Codex, Claude, Cursor, Grok, and Gemini with an API key or
  OAuth sign-in.
- Select a provider and launch a new Claude Code session.
- Start working in the terminal without configuring the proxy by hand.

## Project status

The Tauri application shell is initialized. Provider configuration, the local
proxy, system tray controls, and terminal launching are not implemented yet.

## Development

Install dependencies and start the desktop app:

```sh
pnpm install
pnpm tauri dev
```

Build the desktop binary without packaging an installer:

```sh
pnpm tauri build --debug --no-bundle
```
