// @vitest-environment jsdom

import { render, screen } from "@testing-library/react"
import { describe, expect, it, vi } from "vitest"

import { LaunchOptionsSection } from "@/features/dashboard/components/LaunchOptionsSection"
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

describe("LaunchOptionsSection", () => {
  it("keeps Launch Claude Code while launch is pending", () => {
    render(
      <LaunchOptionsSection
        canLaunch
        pending={TPendingAction.Launching}
        snapshot={snapshot}
        onLaunch={vi.fn(async () => {})}
      />,
    )

    const launchButton = screen.getByRole("button", { name: "Launch Claude Code" })
    expect(launchButton.getAttribute("data-pending")).toBe("true")
    expect(screen.queryByText("Opening")).toBeNull()
  })
})
