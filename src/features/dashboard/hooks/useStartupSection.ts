import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart"
import { toast } from "@heroui/react"
import { useEffect, useState } from "react"

import { STARTUP_FAILED_TOAST } from "@/features/dashboard/constants/dashboardLabels"
import type { ValueChanged } from "@/types"

interface IUseStartupSectionReturn {
  autoStartEnabled: boolean
  isBusy: boolean
  toggle: ValueChanged<boolean, void>
}

/// Reads and switches the login item.
///
/// The operating system owns this state, so the switch reports what the OS
/// answered rather than what was clicked.
export const useStartupSection = () => {
  const [autoStartEnabled, setAutoStartEnabled] = useState(false)
  const [isBusy, setIsBusy] = useState(false)

  useEffect(() => {
    let cancelled = false
    void isEnabled()
      .then((enabled) => {
        if (!cancelled) setAutoStartEnabled(enabled)
      })
      .catch(() => {
        if (!cancelled) toast.danger(STARTUP_FAILED_TOAST)
      })
    return () => {
      cancelled = true
    }
  }, [])

  const toggle = (enabled: boolean) => {
    setIsBusy(true)
    void (enabled ? enable() : disable())
      .then(() => setAutoStartEnabled(enabled))
      .catch(() => toast.danger(STARTUP_FAILED_TOAST))
      .finally(() => setIsBusy(false))
  }

  return { autoStartEnabled, isBusy, toggle } satisfies IUseStartupSectionReturn
}
