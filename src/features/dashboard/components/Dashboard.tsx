import type { FC } from "react"

import { TDashboardStatus, TPendingAction } from "@/features/dashboard/constants/dashboardSchema"
import { useDashboard } from "@/features/dashboard/hooks/useDashboard"
import { ConnectionSection } from "@/features/dashboard/components/ConnectionSection"
import { LaunchSection } from "@/features/dashboard/components/LaunchSection"

export const Dashboard: FC = () => {
  const dashboard = useDashboard()
  const { state } = dashboard

  if (state.status === TDashboardStatus.Loading) {
    return (
      <main className="app-shell">
        <p>Loading settings</p>
      </main>
    )
  }

  if (state.status === TDashboardStatus.Failed) {
    return (
      <main className="app-shell">
        <section className="panel">
          <h1>Launch Claude Code</h1>
          <p role="alert">{state.message}</p>
          <button type="button" onClick={dashboard.reload}>
            Try again
          </button>
        </section>
      </main>
    )
  }

  return (
    <main className="app-shell">
      <header className="page-heading">
        <p>OpenCode Go</p>
        <h1>Launch Claude Code</h1>
      </header>
      {state.errorMessage && <p role="alert">{state.errorMessage}</p>}
      {state.notice && <p role="status">{state.notice}</p>}
      <ConnectionSection
        snapshot={state.snapshot}
        pending={state.pending}
        onSave={dashboard.saveApiKey}
        onDisconnect={dashboard.disconnect}
        onRefresh={dashboard.refreshCatalog}
      />
      <LaunchSection
        snapshot={state.snapshot}
        pending={state.pending}
        onSaveSettings={dashboard.saveSettings}
        onLaunch={dashboard.launch}
      />
      {state.pending !== TPendingAction.None && <p role="status">Working</p>}
    </main>
  )
}
