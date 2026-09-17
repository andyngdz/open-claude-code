// @vitest-environment jsdom

import { act, renderHook } from "@testing-library/react"
import { describe, expect, it, vi } from "vitest"

import { useConnectionSection } from "@/features/dashboard/hooks/useConnectionSection"
import {
  TConnectionStatus,
  TPendingAction,
  TTerminalKind,
  type IDashboardSnapshot,
} from "@/features/dashboard/schemas/dashboard.schema"

const connectedSnapshot = {
  connection: { status: TConnectionStatus.Connected },
  models: [],
  customModels: [],
  aliases: { fable: "model", opus: "model", sonnet: "model", haiku: "model" },
  launchModelId: "model",
  terminal: TTerminalKind.SystemDefault,
  terminals: [],
  lastWorkspace: null,
  catalogRefreshedAtEpochSeconds: null,
  gatewayBaseUrl: "http://127.0.0.1:9",
} satisfies IDashboardSnapshot

describe("useConnectionSection", () => {
  it("fills the API key input from the saved local credential", () => {
    const saveApiKey = vi.fn().mockResolvedValue(true)
    const { result } = renderHook(() => {
      return useConnectionSection(
        connectedSnapshot,
        "saved-api-key",
        TPendingAction.None,
        saveApiKey,
      )
    })

    expect(result.current.getValues("apiKey")).toBe("saved-api-key")
  })

  it("keeps the API key input after a successful save", async () => {
    const saveApiKey = vi.fn().mockResolvedValue(true)
    const { result } = renderHook(() => {
      return useConnectionSection(connectedSnapshot, "", TPendingAction.None, saveApiKey)
    })

    act(() => {
      result.current.setValue("apiKey", "saved-api-key")
    })
    await act(async () => {
      await result.current.onSubmit({ apiKey: "saved-api-key" })
    })

    expect(saveApiKey).toHaveBeenCalledWith("saved-api-key")
    expect(result.current.getValues("apiKey")).toBe("saved-api-key")
  })
})
