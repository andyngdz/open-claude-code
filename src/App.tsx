import { Toast } from "@heroui/react"

import { Dashboard } from "@/features/dashboard/components/Dashboard"
import { DashboardPreview } from "@/features/dashboard/components/DashboardPreview"

/// Renders the dashboard from fixed settings instead of the app's own state.
///
/// Visual work needs a render the browser can reach: outside the desktop shell
/// every invoke fails, so the dashboard only ever shows its failure state. The
/// flag is dev-only, so a built bundle drops the whole branch.
const isPreview = import.meta.env.DEV && new URLSearchParams(window.location.search).has("preview")

/// Which dashboard this entry point renders, picked before the tree is built.
const ActiveDashboard = isPreview ? DashboardPreview : Dashboard

export const App = () => {
  return (
    <div className="min-h-dvh bg-background">
      <ActiveDashboard />
      <Toast.Provider placement="bottom end" />
    </div>
  )
}
