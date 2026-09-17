// @vitest-environment jsdom

import { act, renderHook } from "@testing-library/react"
import { toast } from "@heroui/react"
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest"

import {
  SETTINGS_SAVE_FAILED_NOTICE,
  SETTINGS_SAVED_NOTICE,
} from "@/features/dashboard/constants/dashboardLabels"
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
} satisfies IDashboardSnapshot

describe("useLaunchSection", () => {
  beforeEach(() => {
    vi.spyOn(toast, "success").mockReturnValue({} as never)
    vi.spyOn(toast, "danger").mockReturnValue({} as never)
  })

  afterEach(() => {
    vi.useRealTimers()
    vi.restoreAllMocks()
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
    expect(toast.success).not.toHaveBeenCalled()
    expect(toast.danger).not.toHaveBeenCalled()
  })

  it("toasts Settings saved only after a persist succeeds", async () => {
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
    expect(toast.success).not.toHaveBeenCalled()

    act(() => {
      vi.advanceTimersByTime(350)
    })
    expect(toast.success).not.toHaveBeenCalled()

    await act(async () => {
      finishSave(true)
    })
    expect(toast.success).toHaveBeenCalledWith(SETTINGS_SAVED_NOTICE)
    expect(toast.danger).not.toHaveBeenCalled()
  })

  it("toasts Could not save settings when persist returns false", async () => {
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
    expect(toast.danger).toHaveBeenCalledWith(SETTINGS_SAVE_FAILED_NOTICE)
    expect(toast.success).not.toHaveBeenCalled()
  })

  it("keeps local model edits when a later snapshot repeats persistable settings", async () => {
    vi.useFakeTimers()
    const saveSettings = vi.fn().mockResolvedValue(true)
    const { result, rerender } = renderHook(
      ({ currentSnapshot, pending }) => {
        return useLaunchSection(currentSnapshot, pending, saveSettings)
      },
      { initialProps: { currentSnapshot: snapshot, pending: TPendingAction.None } },
    )

    act(() => {
      result.current.setValue("modelId", "model-b")
      result.current.setValue("customModels", [{ modelId: "" }])
    })
    act(() => {
      vi.advanceTimersByTime(350)
    })
    await vi.waitFor(() => {
      expect(saveSettings).toHaveBeenCalled()
    })

    rerender({
      currentSnapshot: {
        ...snapshot,
        launchModelId: "model-b",
        models: [...snapshot.models, { id: "model-c", displayName: "Model C", isCustom: false }],
      },
      pending: TPendingAction.SavingSettings,
    })

    expect(result.current.getValues("modelId")).toBe("model-b")
    expect(result.current.getValues("customModels")).toEqual([{ modelId: "" }])
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
