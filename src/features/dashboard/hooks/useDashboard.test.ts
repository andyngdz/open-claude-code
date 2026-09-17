// @vitest-environment jsdom

import { act, renderHook } from "@testing-library/react"
import { toast } from "@heroui/react"
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest"

import { useDashboard } from "@/features/dashboard/hooks/useDashboard"
import { dashboardService } from "@/features/dashboard/services/dashboardService"
import {
  TConnectionStatus,
  TDashboardStatus,
  TPendingAction,
  TTerminalKind,
  type IDashboardSnapshot,
} from "@/features/dashboard/schemas/dashboard.schema"

vi.mock("@/features/dashboard/services/dashboardService", () => {
  return {
    dashboardService: {
      loadSnapshot: vi.fn(),
      loadSavedApiKey: vi.fn(),
      saveApiKey: vi.fn(),
      removeCredential: vi.fn(),
      refreshCatalog: vi.fn(),
      saveSettings: vi.fn(),
      chooseWorkspace: vi.fn(),
      launch: vi.fn(),
    },
  }
})

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

const waitUntilReady = async (result: { current: ReturnType<typeof useDashboard> }) => {
  await vi.waitFor(() => {
    expect(result.current.state.status).toBe(TDashboardStatus.Ready)
  })
}

describe("useDashboard", () => {
  beforeEach(() => {
    vi.mocked(dashboardService.loadSnapshot).mockResolvedValue(snapshot)
    vi.mocked(dashboardService.loadSavedApiKey).mockResolvedValue("")
    vi.mocked(dashboardService.refreshCatalog).mockResolvedValue(snapshot)
    vi.mocked(dashboardService.removeCredential).mockResolvedValue(snapshot)
    vi.mocked(dashboardService.saveSettings).mockResolvedValue({
      ...snapshot,
      launchModelId: "model-b",
    })
    vi.spyOn(toast, "success").mockReturnValue({} as never)
    vi.spyOn(toast, "danger").mockReturnValue({} as never)
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  it("toasts Models refreshed after a successful catalog refresh", async () => {
    const { result } = renderHook(() => useDashboard())
    await waitUntilReady(result)

    await act(async () => {
      await result.current.refreshCatalog()
    })

    expect(toast.success).toHaveBeenCalledWith("Models refreshed.")
    expect(result.current.state).not.toHaveProperty("notice")
  })

  it("toasts API key removed after disconnect", async () => {
    const { result } = renderHook(() => useDashboard())
    await waitUntilReady(result)

    await act(async () => {
      await result.current.disconnect()
    })

    expect(toast.success).toHaveBeenCalledWith("API key removed.")
    expect(result.current.state).not.toHaveProperty("notice")
  })

  it("does not replace the dashboard snapshot after saving launch settings", async () => {
    const { result } = renderHook(() => useDashboard())
    await waitUntilReady(result)
    if (result.current.state.status !== TDashboardStatus.Ready) throw new Error("expected ready")
    const snapshotBefore = result.current.state.snapshot

    await act(async () => {
      await result.current.saveSettings({
        terminal: TTerminalKind.SystemDefault,
        modelId: "model-b",
        fable: "model-a",
        opus: "model-a",
        sonnet: "model-a",
        haiku: "model-a",
        customModels: [],
      })
    })

    if (result.current.state.status !== TDashboardStatus.Ready) throw new Error("expected ready")
    expect(result.current.state.pending).toBe(TPendingAction.None)
    expect(result.current.state.snapshot).toBe(snapshotBefore)
  })

  it("toasts catalog refresh failures instead of inline error state", async () => {
    vi.mocked(dashboardService.refreshCatalog).mockRejectedValue(new Error("catalog down"))
    const { result } = renderHook(() => useDashboard())
    await waitUntilReady(result)

    await act(async () => {
      await result.current.refreshCatalog()
    })

    expect(toast.success).not.toHaveBeenCalled()
    expect(toast.danger).toHaveBeenCalledWith("catalog down")
    expect(result.current.state).not.toHaveProperty("errorMessage")
  })

  it("toasts API key load failures without blocking the ready workspace", async () => {
    vi.mocked(dashboardService.loadSavedApiKey).mockRejectedValue(new Error("key store down"))
    const { result } = renderHook(() => useDashboard())
    await waitUntilReady(result)

    expect(toast.danger).toHaveBeenCalledWith("key store down")
    expect(result.current.state).not.toHaveProperty("errorMessage")
  })

  it("toasts when launch is blocked because no API key is saved", async () => {
    vi.mocked(dashboardService.loadSnapshot).mockResolvedValue({
      ...snapshot,
      connection: { status: TConnectionStatus.Disconnected },
    })
    const { result } = renderHook(() => useDashboard())
    await waitUntilReady(result)

    await act(async () => {
      await result.current.launch({
        terminal: TTerminalKind.SystemDefault,
        modelId: "model-a",
        fable: "model-a",
        opus: "model-a",
        sonnet: "model-a",
        haiku: "model-a",
        customModels: [],
      })
    })

    expect(toast.danger).toHaveBeenCalledWith("Save an OpenCode Go API key before launching.")
    expect(result.current.state).not.toHaveProperty("errorMessage")
  })
})
