import { describe, expect, it } from "vitest"

import { customModelIds } from "@/features/dashboard/services/dashboardFormatters"
import { dashboardSnapshotSchema, TConnectionStatus } from "@/features/dashboard/schemas/dashboard.schema"

describe("dashboard snapshot", () => {
  it("parses a disconnected snapshot", () => {
    const snapshot = dashboardSnapshotSchema.parse({
      connection: { status: TConnectionStatus.Disconnected },
      models: [{ id: "qwen3.8-max", displayName: "Qwen 3.8 Max", isCustom: false }],
      customModels: [],
      aliases: { fable: "qwen3.8-max", opus: "qwen3.8-max", sonnet: "qwen3.8-max", haiku: "qwen3.8-flash" },
      terminal: "system_default",
      terminals: [{ kind: "system_default", label: "System default", isAvailable: true }],
      lastWorkspace: null,
      catalogRefreshedAtEpochSeconds: null,
    })
    expect(snapshot.connection.status).toBe(TConnectionStatus.Disconnected)
  })

  it("drops blank custom model inputs", () => {
    expect(
      customModelIds([{ modelId: " custom-beta " }, { modelId: " " }, { modelId: "custom-alpha" }]),
    ).toEqual(["custom-beta", "custom-alpha"])
  })
})
