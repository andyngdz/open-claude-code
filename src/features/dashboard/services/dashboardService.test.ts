import { invoke } from "@tauri-apps/api/core"
import { describe, expect, it, vi } from "vitest"

import {
  TConnectionStatus,
  TTerminalKind,
  type ILaunchForm,
} from "@/features/dashboard/schemas/dashboard.schema"
import { dashboardService } from "@/features/dashboard/services/dashboardService"

vi.mock("@tauri-apps/api/core", () => {
  return { invoke: vi.fn() }
})

const savedSnapshot = {
  connection: { status: TConnectionStatus.Connected },
  models: [],
  customModels: [],
  aliases: {
    fable: "model-a",
    opus: "model-a",
    sonnet: "model-a",
    haiku: "model-a",
    extended: { fable: false, opus: true, sonnet: false, haiku: false },
  },
  launchModelId: "model-a",
  terminal: TTerminalKind.SystemDefault,
  terminals: [],
  lastWorkspace: null,
  catalogRefreshedAtEpochSeconds: null,
}

const launchForm = {
  terminal: TTerminalKind.Ghostty,
  modelId: "model-a",
  fable: "model-a",
  opus: "model-a",
  sonnet: "model-a",
  haiku: "model-a",
  extendedFable: false,
  extendedOpus: true,
  extendedSonnet: false,
  extendedHaiku: false,
  customModels: [{ modelId: "custom-beta" }],
} satisfies ILaunchForm

describe("dashboardService", () => {
  it("sends the alias ticks with the settings it saves", async () => {
    vi.mocked(invoke).mockResolvedValue(savedSnapshot)

    await dashboardService.saveSettings(launchForm)

    // The backend reads a missing `extended` as all unticked, so a payload that
    // drops it would wipe the user's ticks without an error.
    expect(invoke).toHaveBeenCalledWith("save_dashboard_settings", {
      input: {
        terminal: TTerminalKind.Ghostty,
        modelId: "model-a",
        aliases: {
          fable: "model-a",
          opus: "model-a",
          sonnet: "model-a",
          haiku: "model-a",
          extended: { fable: false, opus: true, sonnet: false, haiku: false },
        },
        customModels: ["custom-beta"],
      },
    })
  })
})
