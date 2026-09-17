// @vitest-environment jsdom

import { act, renderHook } from "@testing-library/react"
import { afterEach, describe, expect, it, vi } from "vitest"

import { useLaunchSection } from "@/features/dashboard/hooks/useLaunchSection"
import { TAutosaveStatus } from "@/features/dashboard/constants/dashboardLabels"
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

  it("reports saved only after a persist succeeds", async () => {
    vi.useFakeTimers()
    let finishSave: (saved: boolean) => void = () => {}
    const saveSettings = vi.fn(
      () =>
        new Promise<boolean>((resolve) => {
          finishSave = resolve
        }),
    )
    const { result } = renderHook(() => {
      return useLaunchSection(snapshot, TPendingAction.None, saveSettings)
    })

    expect(result.current.autosaveStatus).toBe(TAutosaveStatus.Idle)

    act(() => {
      result.current.setValue("modelId", "model-b")
    })
    expect(result.current.autosaveStatus).toBe(TAutosaveStatus.Idle)

    act(() => {
      vi.advanceTimersByTime(350)
    })
    expect(result.current.autosaveStatus).toBe(TAutosaveStatus.Saving)

    await act(async () => {
      finishSave(true)
    })
    expect(result.current.autosaveStatus).toBe(TAutosaveStatus.Saved)
  })

  it("reports save failed when persist returns false", async () => {
    vi.useFakeTimers()
    let finishSave: (saved: boolean) => void = () => {}
    const saveSettings = vi.fn(
      () =>
        new Promise<boolean>((resolve) => {
          finishSave = resolve
        }),
    )
    const { result } = renderHook(() => {
      return useLaunchSection(snapshot, TPendingAction.None, saveSettings)
    })

    act(() => {
      result.current.setValue("modelId", "model-b")
    })
    act(() => {
      vi.advanceTimersByTime(350)
    })

    await act(async () => {
      finishSave(false)
    })
    expect(result.current.autosaveStatus).toBe(TAutosaveStatus.Failed)
  })

  it("keeps an empty custom model row instead of autosaving it away", async () => {
    vi.useFakeTimers()
    const saveSettings = vi.fn().mockResolvedValue(true)
    const { result, rerender } = renderHook(
      ({ currentSnapshot }) => {
        return useLaunchSection(currentSnapshot, TPendingAction.None, saveSettings)
      },
      { initialProps: { currentSnapshot: snapshot } },
    )

    act(() => {
      result.current.setValue("customModels", [{ modelId: "" }])
    })
    act(() => {
      vi.advanceTimersByTime(350)
    })
    await Promise.resolve()

    expect(saveSettings).not.toHaveBeenCalled()
    expect(result.current.getValues("customModels")).toEqual([{ modelId: "" }])

    rerender({ currentSnapshot: { ...snapshot } })
    expect(result.current.getValues("customModels")).toEqual([{ modelId: "" }])
  })
})
