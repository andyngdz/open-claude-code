import { useCallback, useEffect, useState } from "react"
import { toast } from "@heroui/react"
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
      apiKey: string
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
  saveSettings: ValueChanged<ILaunchForm, Promise<boolean>>
  launch: ValueChanged<ILaunchForm, Promise<void>>
}

export const useDashboard = () => {
  const [state, setState] = useState<TDashboardState>({ status: TDashboardStatus.Loading })
  const [reloadCount, setReloadCount] = useState(0)

  const loadSnapshot = useCallback(async () => {
    setState({ status: TDashboardStatus.Loading })
    try {
      const [snapshotResult, apiKeyResult] = await Promise.allSettled([
        dashboardService.loadSnapshot(),
        dashboardService.loadSavedApiKey(),
      ])
      if (snapshotResult.status === "rejected") throw snapshotResult.reason

      const apiKey = apiKeyResult.status === "fulfilled" ? apiKeyResult.value : ""
      setState({
        status: TDashboardStatus.Ready,
        snapshot: snapshotResult.value,
        apiKey,
        pending: TPendingAction.None,
        ...(apiKeyResult.status === "rejected" && {
          errorMessage: readCommandError(apiKeyResult.reason),
        }),
      })
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
        setState((current) => {
          if (current.status !== TDashboardStatus.Ready) return current
          return {
            ...current,
            snapshot,
            pending: TPendingAction.None,
            notice,
          }
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
      const saved = await runAction(
        TPendingAction.SavingKey,
        () => dashboardService.saveApiKey(apiKey),
      )
      if (saved) {
        setState((current) => {
          if (current.status !== TDashboardStatus.Ready) return current
          return { ...current, apiKey }
        })
        toast.success("API key saved.")
      }
      return saved
    },
    [runAction],
  )

  const disconnect = useCallback(async () => {
    const disconnected = await runAction(
      TPendingAction.Disconnecting,
      () => dashboardService.removeCredential(),
      "API key removed.",
    )
    if (!disconnected) return
    setState((current) => {
      if (current.status !== TDashboardStatus.Ready) return current
      return { ...current, apiKey: "" }
    })
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
      return await runAction(
        TPendingAction.SavingSettings,
        () => dashboardService.saveSettings(values),
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
        apiKey: state.apiKey,
        pending: TPendingAction.None,
        errorMessage: "Save an OpenCode Go API key before launching.",
      })
      return
    }
    setState({
      status: TDashboardStatus.Ready,
      snapshot: state.snapshot,
      apiKey: state.apiKey,
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
