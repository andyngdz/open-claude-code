// @vitest-environment jsdom

import { render, screen } from "@testing-library/react"
import { describe, expect, it, vi } from "vitest"

import { ConnectionSection } from "@/features/dashboard/components/ConnectionSection"
import {
  TConnectionStatus,
  TPendingAction,
  TTerminalKind,
  type IDashboardSnapshot,
} from "@/features/dashboard/schemas/dashboard.schema"

const snapshot = {
  connection: { status: TConnectionStatus.Connected },
  models: [{ id: "model-a", displayName: "Model A", isCustom: false }],
  customModels: [],
  aliases: { fable: "model-a", opus: "model-a", sonnet: "model-a", haiku: "model-a" },
  launchModelId: "model-a",
  terminal: TTerminalKind.SystemDefault,
  terminals: [],
  lastWorkspace: null,
  catalogRefreshedAtEpochSeconds: null,
} satisfies IDashboardSnapshot

const noopAsync = vi.fn(async () => {})
const saveApiKey = vi.fn(async () => true)

describe("ConnectionSection", () => {
  it("keeps Save API key while the save is pending", () => {
    render(
      <ConnectionSection
        apiKey="saved-api-key"
        pending={TPendingAction.SavingKey}
        snapshot={snapshot}
        onDisconnect={noopAsync}
        onRefresh={noopAsync}
        onSaveApiKey={saveApiKey}
      />,
    )

    const saveButton = screen.getByRole("button", { name: "Save API key" })
    expect(saveButton.getAttribute("data-pending")).toBe("true")
    expect(screen.queryByText("Saving")).toBeNull()
  })
})
