import { useMemo, useState } from "react"
import type { FC, ReactNode } from "react"

import { ProviderPanel } from "@/features/dashboard/components/ProviderPanel"
import { SettingsPanel } from "@/features/dashboard/components/SettingsPanel"
import { TProviderId } from "@/features/dashboard/constants/dashboardProviders"
import {
  isSettingsView,
  type TSidebarSelection,
} from "@/features/dashboard/constants/dashboardNavigation"
import { DashboardSidebar } from "@/features/dashboard/presentations/DashboardSidebar"

interface IDashboardShellProps {
  children: ReactNode
}

export const DashboardShell: FC<IDashboardShellProps> = ({
  children,
}) => {
  /// One selection covers the provider rows and the Settings entry, so the two
  /// panels can never be open together.
  const [selection, setSelection] = useState<TSidebarSelection>(TProviderId.OpenCode)

  const panel = useMemo(() => {
    if (isSettingsView(selection)) return <SettingsPanel />
    return <ProviderPanel provider={selection}>{children}</ProviderPanel>
  }, [children, selection])

  return (
    <div className="min-h-dvh bg-background text-foreground md:h-dvh md:overflow-hidden md:flex">
      <DashboardSidebar selected={selection} onSelect={setSelection} />
      <div className="min-w-0 flex-1 md:min-h-0 md:overflow-y-auto">
        {panel}
      </div>
    </div>
  )
}
