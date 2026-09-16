import { useCallback, useEffect, useState } from "react"

import { readCommandError } from "@/features/dashboard/services/dashboardFormatters"
import type { IClaudeLaunch } from "@/features/dashboard/interfaces/dashboardService"
import { dashboardService } from "@/features/dashboard/services/dashboardService"
import {
  TDashboardStatus,
  TPendingAction,
  type IDashboardSnapshot,
  type ILaunchForm,
} from "@/features/dashboard/schemas/dashboard.schema"
import type { ValueChanged } from "@/types"

type TDashboardState =
  | { status: TDashboardStatus.Loading }
  | { status: TDashboardStatus.Failed; message: string }
  | {
      status: TDashboardStatus.Ready
      snapshot: IDashboardSnapshot
      pending: TPendingAction
      notice?: string
      errorMessage?: string
    }

interface IUseDashboardReturn {
  state: TDashboardState
  reload: () => void
  saveApiKey: ValueChanged<string, Promise<void>>
  disconnect: () => Promise<void>
  refreshCatalog: () => Promise<void>
  saveSettings: ValueChanged<ILaunchForm, Promise<void>>
  launch: ValueChanged<IClaudeLaunch, Promise<void>>
}

export const useDashboard = () => {
  const [state, setState] = useState<TDashboardState>({ status: TDashboardStatus.Loading })
  const [reloadCount, setReloadCount] = useState(0)

  const loadSnapshot = useCallback(async () => {
    setState({ status: TDashboardStatus.Loading })
    try {
      const snapshot = await dashboardService.loadSnapshot()
      setState({ status: TDashboardStatus.Ready, snapshot, pending: TPendingAction.None })
    } catch (error) {
      setState({ status: TDashboardStatus.Failed, message: readCommandError(error) })
    }
  }, [])

  const runAction = useCallback(
    async (pending: TPendingAction, action: () => Promise<IDashboardSnapshot>, notice?: string) => {
      setState((current) => {
        if (current.status !== TDashboardStatus.Ready) return current
        return { ...current, pending, errorMessage: undefined, notice: undefined }
      })
      try {
        const snapshot = await action()
        setState({
          status: TDashboardStatus.Ready,
          snapshot,
          pending: TPendingAction.None,
          notice,
        })
        return true
      } catch (error) {
        const message = readCommandError(error)
        setState((current) => {
          if (current.status !== TDashboardStatus.Ready) {
            return { status: TDashboardStatus.Failed, message }
          }
          return { ...current, pending: TPendingAction.None, errorMessage: message }
        })
        return false
      }
    },
    [],
  )

  const reload = useCallback(() => {
    setReloadCount((current) => current + 1)
  }, [])

  useEffect(() => {
    void loadSnapshot()
  }, [loadSnapshot, reloadCount])

  const saveApiKey = useCallback(
    async (apiKey: string) => {
      await runAction(TPendingAction.SavingKey, () => dashboardService.saveApiKey(apiKey), "API key saved.")
    },
    [runAction],
  )

  const disconnect = useCallback(async () => {
    await runAction(
      TPendingAction.Disconnecting,
      () => dashboardService.removeCredential(),
      "API key removed.",
    )
  }, [runAction])

  const refreshCatalog = useCallback(async () => {
    await runAction(
      TPendingAction.Refreshing,
      () => dashboardService.refreshCatalog(),
      "Models refreshed.",
    )
  }, [runAction])

  const saveSettings = useCallback(
    async (values: ILaunchForm) => {
      await runAction(
        TPendingAction.SavingSettings,
        () => dashboardService.saveSettings(values),
        "Settings saved.",
      )
    },
    [runAction],
  )

  const launch = useCallback(
    async (request: IClaudeLaunch) => {
      const saved = await runAction(
        TPendingAction.SavingSettings,
        () => dashboardService.saveSettings(request.settings),
      )
      if (!saved) return
      await runAction(
        TPendingAction.Launching,
        () => dashboardService.launch(request.settings, request.workspace),
        "Claude Code opened.",
      )
    },
    [runAction],
  )

  return {
    state,
    reload,
    saveApiKey,
    disconnect,
    refreshCatalog,
    saveSettings,
    launch,
  } satisfies IUseDashboardReturn
}
