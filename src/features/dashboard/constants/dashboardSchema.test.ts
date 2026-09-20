import { omit } from "es-toolkit/compat"
import { describe, expect, it } from "vitest"

import { customModelIds } from "@/features/dashboard/services/dashboardFormatters"
import { dashboardSnapshotSchema, TConnectionStatus } from "@/features/dashboard/schemas/dashboard.schema"

const snapshotPayload = {
  connection: { status: TConnectionStatus.Disconnected },
  models: [{ id: "qwen3.8-max", displayName: "Qwen 3.8 Max", isCustom: false }],
  customModels: [],
  aliases: {
    fable: "qwen3.8-max",
    opus: "qwen3.8-max",
    sonnet: "qwen3.8-max",
    haiku: "qwen3.8-flash",
    extended: { fable: true, opus: false, sonnet: false, haiku: false },
  },
  launchModelId: "qwen3.8-max",
  terminal: "system_default",
  terminals: [{ kind: "system_default", label: "System default", isAvailable: true }],
  lastWorkspace: null,
  catalogRefreshedAtEpochSeconds: null,
}

describe("dashboard snapshot", () => {
  it("parses a disconnected snapshot", () => {
    const snapshot = dashboardSnapshotSchema.parse(snapshotPayload)
    expect(snapshot.connection.status).toBe(TConnectionStatus.Disconnected)
  })

  it("keeps the alias ticks the snapshot carries", () => {
    const snapshot = dashboardSnapshotSchema.parse(snapshotPayload)
    expect(snapshot.aliases.extended).toEqual({ fable: true, opus: false, sonnet: false, haiku: false })
  })

  it("refuses a snapshot that dropped the alias ticks", () => {
    const payload = { ...snapshotPayload, aliases: omit(snapshotPayload.aliases, ["extended"]) }

    expect(dashboardSnapshotSchema.safeParse(payload).success).toBe(false)
  })

  it("drops blank custom model inputs", () => {
    expect(
      customModelIds([{ modelId: " custom-beta " }, { modelId: " " }, { modelId: "custom-alpha" }]),
    ).toEqual(["custom-beta", "custom-alpha"])
  })
})
