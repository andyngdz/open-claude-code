// @vitest-environment jsdom

import { act, renderHook } from "@testing-library/react"
import { afterEach, describe, expect, it, vi } from "vitest"

import { useLaunchSection } from "@/features/dashboard/hooks/useLaunchSection"
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
  gatewayBaseUrl: "http://127.0.0.1:9",
} satisfies IDashboardSnapshot

describe("useLaunchSection", () => {
  afterEach(() => {
    vi.useRealTimers()
  })

  it("saves changed settings without requiring a manual submit", async () => {
    vi.useFakeTimers()
    const saveSettings = vi.fn().mockResolvedValue(true)
    const { result } = renderHook(() => {
      return useLaunchSection(snapshot, TPendingAction.None, saveSettings)
    })

    act(() => {
      result.current.setValue("modelId", "model-b")
      vi.advanceTimersByTime(350)
    })

    await vi.waitFor(() => {
      expect(saveSettings).toHaveBeenCalledWith(
        expect.objectContaining({ modelId: "model-b" }),
      )
    })
  })

  it("does not save settings loaded from the current snapshot", () => {
    vi.useFakeTimers()
    const saveSettings = vi.fn().mockResolvedValue(true)
    renderHook(() => {
      return useLaunchSection(snapshot, TPendingAction.None, saveSettings)
    })

    act(() => {
      vi.advanceTimersByTime(350)
    })

    expect(saveSettings).not.toHaveBeenCalled()
  })
})
