// @vitest-environment jsdom

import { toast } from "@heroui/react"
import { act, renderHook, waitFor } from "@testing-library/react"
import { afterEach, describe, expect, it, vi } from "vitest"

import { STARTUP_FAILED_TOAST } from "@/features/dashboard/constants/dashboardLabels"
import { useStartupSection } from "@/features/dashboard/hooks/useStartupSection"

const disable = vi.fn<() => Promise<void>>()
const enable = vi.fn<() => Promise<void>>()
const isEnabled = vi.fn<() => Promise<boolean>>()

vi.mock("@tauri-apps/plugin-autostart", () => ({
  disable: () => disable(),
  enable: () => enable(),
  isEnabled: () => isEnabled(),
}))

describe("useStartupSection", () => {
  afterEach(() => {
    vi.restoreAllMocks()
  })

  it("reports the switch state the operating system holds", async () => {
    isEnabled.mockResolvedValue(true)

    const { result } = renderHook(() => useStartupSection())

    await waitFor(() => {
      expect(result.current.autoStartEnabled).toBe(true)
    })
  })

  it("enables the login item when the switch turns on", async () => {
    isEnabled.mockResolvedValue(false)
    enable.mockResolvedValue(undefined)
    const { result } = renderHook(() => useStartupSection())
    await waitFor(() => {
      expect(result.current.isBusy).toBe(false)
    })

    act(() => {
      result.current.toggle(true)
    })

    await waitFor(() => {
      expect(enable).toHaveBeenCalledTimes(1)
      expect(result.current.autoStartEnabled).toBe(true)
    })
    expect(disable).not.toHaveBeenCalled()
  })

  it("disables the login item when the switch turns off", async () => {
    isEnabled.mockResolvedValue(true)
    disable.mockResolvedValue(undefined)
    const { result } = renderHook(() => useStartupSection())
    await waitFor(() => {
      expect(result.current.autoStartEnabled).toBe(true)
    })

    act(() => {
      result.current.toggle(false)
    })

    await waitFor(() => {
      expect(disable).toHaveBeenCalledTimes(1)
      expect(result.current.autoStartEnabled).toBe(false)
    })
    expect(enable).not.toHaveBeenCalled()
  })

  it("keeps the switch where it was when the operating system refuses", async () => {
    isEnabled.mockResolvedValue(false)
    enable.mockRejectedValue(new Error("refused"))
    const danger = vi.spyOn(toast, "danger").mockReturnValue("toast-id" as never)
    const { result } = renderHook(() => useStartupSection())
    await waitFor(() => {
      expect(result.current.isBusy).toBe(false)
    })

    act(() => {
      result.current.toggle(true)
    })

    await waitFor(() => {
      expect(danger).toHaveBeenCalledWith(STARTUP_FAILED_TOAST)
    })
    expect(result.current.autoStartEnabled).toBe(false)
    expect(result.current.isBusy).toBe(false)
  })
})
