// @vitest-environment jsdom

import { toast } from "@heroui/react"
import { act, renderHook } from "@testing-library/react"
import { afterEach, describe, expect, it, vi } from "vitest"

import {
  AUTOSAVE_FAILED_TOAST,
  AUTOSAVE_SAVED_TOAST,
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
  aliases: {
    fable: "model-a",
    opus: "model-a",
    sonnet: "model-a",
    haiku: "model-a",
    extended: { fable: false, opus: false, sonnet: false, haiku: false },
  },
  launchModelId: "model-a",
  terminal: TTerminalKind.SystemDefault,
  terminals: [],
  lastWorkspace: null,
  catalogRefreshedAtEpochSeconds: null,
} satisfies IDashboardSnapshot

describe("useLaunchSection", () => {
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
  })

  it("toasts Saved only after a persist succeeds", async () => {
    vi.useFakeTimers()
    vi.spyOn(toast, "success").mockReturnValue("toast-id" as never)
    vi.spyOn(toast, "danger").mockReturnValue("toast-id" as never)
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
    expect(toast.success).toHaveBeenCalledWith(AUTOSAVE_SAVED_TOAST)
    expect(toast.danger).not.toHaveBeenCalled()
  })

  it("toasts Save failed when persist returns false", async () => {
    vi.useFakeTimers()
    vi.spyOn(toast, "success").mockReturnValue("toast-id" as never)
    vi.spyOn(toast, "danger").mockReturnValue("toast-id" as never)
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
    expect(toast.danger).toHaveBeenCalledWith(AUTOSAVE_FAILED_TOAST)
    expect(toast.success).not.toHaveBeenCalled()
  })

  it("starts from the ticks the snapshot carries", () => {
    const ticked = {
      ...snapshot,
      aliases: {
        ...snapshot.aliases,
        extended: { ...snapshot.aliases.extended, opus: true },
      },
    } satisfies IDashboardSnapshot
    const { result } = renderHook(() => {
      return useLaunchSection(ticked, TPendingAction.None, vi.fn().mockResolvedValue(true))
    })

    expect(result.current.getValues("extendedOpus")).toBe(true)
    expect(result.current.getValues("extendedFable")).toBe(false)
  })

  it("autosaves a ticked alias row", async () => {
    vi.useFakeTimers()
    const saveSettings = vi.fn().mockResolvedValue(true)
    const { result } = renderHook(() => {
      return useLaunchSection(snapshot, TPendingAction.None, saveSettings)
    })

    act(() => {
      result.current.setValue("extendedOpus", true)
      vi.advanceTimersByTime(350)
    })

    await vi.waitFor(() => {
      expect(saveSettings).toHaveBeenCalledWith(
        expect.objectContaining({ extendedOpus: true }),
      )
    })
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
