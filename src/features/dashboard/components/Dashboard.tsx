import { Button } from "@heroui/react"
import type { FC } from "react"

import { ConnectionSection } from "@/features/dashboard/components/ConnectionSection"
import { DashboardShell } from "@/features/dashboard/components/DashboardShell"
import { LaunchSection } from "@/features/dashboard/components/LaunchSection"
import { TDashboardAlertStatus } from "@/features/dashboard/constants/dashboardLabels"
import { useDashboard } from "@/features/dashboard/hooks/useDashboard"
import { DashboardAlert } from "@/features/dashboard/presentations/DashboardAlert"
import { StatusStrip } from "@/features/dashboard/presentations/StatusStrip"
import { TDashboardStatus } from "@/features/dashboard/schemas/dashboard.schema"

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
        <h1 className="text-lg font-semibold">OpenCode Go</h1>
        <DashboardAlert message={state.message} status={TDashboardAlertStatus.Danger} />
        <Button className="rounded-md" type="button" onPress={dashboard.reload}>
          Try again
        </Button>
      </DashboardShell>
    )
  }

  return (
    <DashboardShell>
      <header className="py-2">
        <h1 className="text-4xl font-bold leading-tight text-foreground">OpenCode Go</h1>
      </header>
      <StatusStrip providerLabel="OpenCode Go" snapshot={state.snapshot} />
      {state.errorMessage && (
        <DashboardAlert message={state.errorMessage} status={TDashboardAlertStatus.Danger} />
      )}
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
    </DashboardShell>
  )
}
