import { Button } from "@heroui/react"
import type { FC } from "react"

import { ConnectionSection } from "@/features/dashboard/components/ConnectionSection"
import { DashboardShell } from "@/features/dashboard/components/DashboardShell"
import { LaunchSection } from "@/features/dashboard/components/LaunchSection"
import { TDashboardAlertStatus } from "@/features/dashboard/constants/dashboardLabels"
import { useDashboard } from "@/features/dashboard/hooks/useDashboard"
import { DashboardAlert } from "@/features/dashboard/presentations/DashboardAlert"
import { StatusStrip } from "@/features/dashboard/presentations/StatusStrip"
import { TDashboardStatus, TPendingAction } from "@/features/dashboard/schemas/dashboard.schema"

export const Dashboard: FC = () => {
  const dashboard = useDashboard()
  const { state } = dashboard

  if (state.status === TDashboardStatus.Loading) {
    return (
      <DashboardShell>
        <p className="text-sm text-muted">Loading settings</p>
      </DashboardShell>
    )
  }

  if (state.status === TDashboardStatus.Failed) {
    return (
      <DashboardShell>
        <h1 className="text-lg font-semibold">Launch Claude Code</h1>
        <DashboardAlert message={state.message} status={TDashboardAlertStatus.Danger} />
        <Button type="button" onPress={dashboard.reload}>
          Try again
        </Button>
      </DashboardShell>
    )
  }

  const showsWorkingAlert =
    state.pending !== TPendingAction.None && state.pending !== TPendingAction.SavingSettings

  return (
    <DashboardShell>
      <header className="workspace-header">
        <div>
          <h1 className="workspace-title">OpenCode Go</h1>
          <p className="workspace-subtitle">Launch Claude Code</p>
          <p className="workspace-copy">
            Configure the connection, choose models, and launch Claude Code through the local gateway.
          </p>
        </div>
      </header>
      <StatusStrip providerLabel="OpenCode Go" snapshot={state.snapshot} />
      {state.errorMessage && (
        <DashboardAlert message={state.errorMessage} status={TDashboardAlertStatus.Danger} />
      )}
      {state.notice && <DashboardAlert message={state.notice} status={TDashboardAlertStatus.Success} />}
      <ConnectionSection
        apiKey={state.apiKey}
        pending={state.pending}
        snapshot={state.snapshot}
        onDisconnect={dashboard.disconnect}
        onRefresh={dashboard.refreshCatalog}
        onSaveApiKey={dashboard.saveApiKey}
      />
      <LaunchSection
        pending={state.pending}
        snapshot={state.snapshot}
        onSaveSettings={dashboard.saveSettings}
        onLaunch={dashboard.launch}
      />
      {showsWorkingAlert && (
        <DashboardAlert message="Working" status={TDashboardAlertStatus.Accent} />
      )}
    </DashboardShell>
  )
}
