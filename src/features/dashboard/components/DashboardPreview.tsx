import type { FC } from "react"

import { ConnectionSection } from "@/features/dashboard/components/ConnectionSection"
import { DashboardShell } from "@/features/dashboard/components/DashboardShell"
import { LaunchSection } from "@/features/dashboard/components/LaunchSection"
import {
  TConnectionStatus,
  TPendingAction,
  TTerminalKind,
  type IDashboardSnapshot,
  type ILaunchForm,
} from "@/features/dashboard/schemas/dashboard.schema"

const previewSnapshot = {
  connection: { status: TConnectionStatus.Connected },
  models: [
    { id: "qwen3.8-max", displayName: "Qwen 3.8 Max", isCustom: false },
    { id: "qwen3.8-flash", displayName: "Qwen 3.8 Flash", isCustom: false },
  ],
  customModels: ["custom-beta"],
  aliases: {
    fable: "qwen3.8-max",
    opus: "qwen3.8-max",
    sonnet: "qwen3.8-max",
    haiku: "qwen3.8-flash",
  },
  launchModelId: "qwen3.8-max",
  terminal: TTerminalKind.SystemDefault,
  terminals: [
    { kind: TTerminalKind.SystemDefault, label: "System default", isAvailable: true },
    { kind: TTerminalKind.Ghostty, label: "Ghostty", isAvailable: true },
    { kind: TTerminalKind.GnomeTerminal, label: "GNOME Terminal", isAvailable: false },
  ],
  lastWorkspace: "/workspace/example-project",
  catalogRefreshedAtEpochSeconds: null,
  gatewayBaseUrl: "http://127.0.0.1:9",
} satisfies IDashboardSnapshot

export const DashboardPreview: FC = () => {
  const saveApiKey = async () => true
  const saveSettings = async (_values: ILaunchForm) => true
  const launch = async () => {}

  return (
    <DashboardShell>
      <header className="flex flex-col gap-2">
        <p className="text-sm text-muted">OpenCode Go</p>
        <h1 className="text-lg font-semibold">Launch Claude Code</h1>
      </header>
      <ConnectionSection
        apiKey=""
        pending={TPendingAction.None}
        snapshot={previewSnapshot}
        onDisconnect={async () => {}}
        onRefresh={async () => {}}
        onSaveApiKey={saveApiKey}
      />
      <LaunchSection
        pending={TPendingAction.None}
        snapshot={previewSnapshot}
        onSaveSettings={saveSettings}
        onLaunch={launch}
      />
    </DashboardShell>
  )
}
