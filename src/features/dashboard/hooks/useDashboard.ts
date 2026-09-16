import { useCallback, useEffect, useState } from "react"
import { isString } from "es-toolkit/compat"

import { readCommandError } from "@/features/dashboard/services/dashboardFormatters"
import { dashboardService } from "@/features/dashboard/services/dashboardService"
import {
  TConnectionStatus,
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
  saveApiKey: ValueChanged<string, Promise<boolean>>
  disconnect: () => Promise<void>
  refreshCatalog: () => Promise<void>
  saveSettings: ValueChanged<ILaunchForm, Promise<void>>
  launch: ValueChanged<ILaunchForm, Promise<void>>
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
      return runAction(TPendingAction.SavingKey, () => dashboardService.saveApiKey(apiKey), "API key saved.")
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

  const launch = useCallback(async (settings: ILaunchForm) => {
    if (state.status !== TDashboardStatus.Ready) return
    if (state.snapshot.connection.status !== TConnectionStatus.Connected) {
      setState({
        status: TDashboardStatus.Ready,
        snapshot: state.snapshot,
        pending: TPendingAction.None,
        errorMessage: "Save an OpenCode Go API key before launching.",
      })
      return
    }
    setState({
      status: TDashboardStatus.Ready,
      snapshot: state.snapshot,
      pending: TPendingAction.Launching,
    })
    let workspace: string | undefined
    try {
      const lastWorkspace = state.snapshot.lastWorkspace
      if (isString(lastWorkspace)) {
        workspace = await dashboardService.chooseWorkspace(lastWorkspace)
      } else {
        workspace = await dashboardService.chooseWorkspace()
      }
    } catch (error) {
      const message = readCommandError(error)
      setState((current) => {
        if (current.status !== TDashboardStatus.Ready) {
          return { status: TDashboardStatus.Failed, message }
        }
        return { ...current, pending: TPendingAction.None, errorMessage: message }
      })
      return
    }
    if (!workspace) {
      setState((current) => {
        if (current.status !== TDashboardStatus.Ready) return current
        return { ...current, pending: TPendingAction.None }
      })
      return
    }
    const saved = await runAction(
      TPendingAction.SavingSettings,
      () => dashboardService.saveSettings(settings),
    )
    if (!saved) return
    await runAction(
      TPendingAction.Launching,
      () => dashboardService.launch(settings, workspace),
      "Claude Code opened.",
    )
  }, [runAction, state])

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
