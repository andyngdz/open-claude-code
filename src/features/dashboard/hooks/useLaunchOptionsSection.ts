import { open } from "@tauri-apps/plugin-dialog"
import { isString } from "es-toolkit/compat"
import { useEffect, useState } from "react"

import { LAUNCH_COMMAND } from "@/features/dashboard/constants/dashboardLabels"
import type { IClaudeLaunch } from "@/features/dashboard/interfaces/dashboardService"
import { TConnectionStatus, TPendingAction, type IDashboardSnapshot } from "@/features/dashboard/schemas/dashboard.schema"
import { launchFormDefaults } from "@/features/dashboard/services/dashboardFormatters"
import type { ValueChanged } from "@/types"

const COPIED_RESET_MS = 2000

interface IUseLaunchOptionsSectionReturn {
  canPressLaunch: boolean
  copied: boolean
  isLaunching: boolean
  onCopy: () => Promise<void>
  onPressLaunch: () => Promise<void>
}

export const useLaunchOptionsSection = (
  snapshot: IDashboardSnapshot,
  pending: TPendingAction,
  canLaunch: boolean,
  onLaunch: ValueChanged<IClaudeLaunch, Promise<void>>,
) => {
  const [copied, setCopied] = useState(false)
  const isBusy = pending !== TPendingAction.None

  useEffect(() => {
    if (!copied) return
    const timer = window.setTimeout(() => {
      setCopied(false)
    }, COPIED_RESET_MS)
    return () => {
      window.clearTimeout(timer)
    }
  }, [copied])

  const onCopy = async () => {
    if (import.meta.env.SSR) return
    try {
      await navigator.clipboard.writeText(LAUNCH_COMMAND)
      setCopied(true)
    } catch {
      setCopied(false)
    }
  }

  const onPressLaunch = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Choose folder",
      ...(isString(snapshot.lastWorkspace) ? { defaultPath: snapshot.lastWorkspace } : {}),
    })
    if (!isString(selected)) return
    await onLaunch({ settings: launchFormDefaults(snapshot), workspace: selected })
  }

  return {
    canPressLaunch:
      canLaunch && snapshot.connection.status === TConnectionStatus.Connected && !isBusy,
    copied,
    isLaunching: pending === TPendingAction.Launching,
    onCopy,
    onPressLaunch,
  } satisfies IUseLaunchOptionsSectionReturn
}
