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

The Linux desktop app runs from the system tray, can save an OpenCode Go API
key, choose a model and workspace, and open Claude Code through a local
gateway. Closing the settings window keeps the gateway running; choose Quit
from the tray menu to stop it. Other providers are not implemented yet.

## Install

Install the latest release on macOS or Linux:

```sh
curl -fsSL https://andyngdz.github.io/open-claude-code/install.sh | sh
```

The script installs DEB or RPM packages when supported. On other x64 Linux
distributions, it installs an AppImage in `~/.local/bin` and adds Open Claude
Code, with its icon, to your app launcher. For Windows installers and manual
downloads, see the [installation guide](https://andyngdz.github.io/open-claude-code/).

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
