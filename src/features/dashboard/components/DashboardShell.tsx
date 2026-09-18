import { useState } from "react"
import type { FC, ReactNode } from "react"

import { ProviderPanel } from "@/features/dashboard/components/ProviderPanel"
import { TProviderId } from "@/features/dashboard/constants/dashboardProviders"
import { ProviderSidebar } from "@/features/dashboard/presentations/ProviderSidebar"

interface IDashboardShellProps {
  children: ReactNode
}

export const DashboardShell: FC<IDashboardShellProps> = ({
  children,
}) => {
  const [provider, setProvider] = useState(TProviderId.OpenCode)

  return (
    <div className="min-h-dvh bg-background text-foreground md:h-dvh md:overflow-hidden md:flex">
      <ProviderSidebar selected={provider} onSelect={setProvider} />
      <div className="min-w-0 flex-1 md:min-h-0 md:overflow-y-auto">
        <ProviderPanel provider={provider}>
          {children}
        </ProviderPanel>
      </div>
    </div>
  )
}
