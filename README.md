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
distributions, it installs an AppImage in `~/.local/bin`, a command named
`open-claude-code`, and an app launcher entry. If your shell cannot find that
command, add `~/.local/bin` to `PATH`. For Windows installers and manual
downloads, see the [installation guide](https://andyngdz.github.io/open-claude-code/).

After the app is running, start a session from a terminal:

```sh
open-claude-code launch
```

The model you pick at the prompt is remembered for the next terminal launch,
separately from the dashboard's Default model. Flags after `--` go to Claude Code
unchanged, after the model flag:

```sh
open-claude-code launch -- -p "how does this repository work?"
```

Each of the four alias rows in the dashboard has a 1M context box. A model runs
with a 1M window when a ticked row points at it, whether Claude Code reaches it
as an alias or as the Default model. The tick only declares the window to Claude
Code; it does not change what the provider serves, so tick a model you know
offers 1M. Below 1M the request fails once the conversation passes the real
limit, instead of compacting.

A model named on the command line takes the same marker, which Claude Code
strips before the request reaches the local gateway:

```sh
open-claude-code launch --model 'deepseek-v4.1-flash[1m]'
```

Claude Code marks such a request with the `context-1m-2025-08-07` beta header,
and the gateway forwards beta headers unchanged.

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
