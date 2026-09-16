import { useState } from "react"
import type { FC, ReactNode } from "react"

import { ProviderPanel } from "@/features/dashboard/components/ProviderPanel"
import { TProviderId } from "@/features/dashboard/constants/dashboardProviders"
import { ProviderSidebar } from "@/features/dashboard/presentations/ProviderSidebar"
import { TPendingAction, type IDashboardSnapshot } from "@/features/dashboard/schemas/dashboard.schema"

interface IDashboardShellProps {
  children: ReactNode
  snapshot?: IDashboardSnapshot
  pending?: TPendingAction
  onLaunch?: () => Promise<void>
}

export const DashboardShell: FC<IDashboardShellProps> = ({
  children,
  snapshot,
  pending,
  onLaunch,
}) => {
  const [provider, setProvider] = useState(TProviderId.OpenCode)

  return (
    <div className="app-frame">
      <ProviderSidebar selected={provider} onSelect={setProvider} />
      <div className="min-w-0 flex-1">
        <ProviderPanel
          pending={pending}
          provider={provider}
          snapshot={snapshot}
          onLaunch={onLaunch}
        >
          {children}
        </ProviderPanel>
      </div>
    </div>
  )
}
