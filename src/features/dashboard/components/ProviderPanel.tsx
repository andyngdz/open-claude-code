import { useMemo } from "react"
import type { FC, ReactNode } from "react"

import { LaunchOptionsSection } from "@/features/dashboard/components/LaunchOptionsSection"
import {
  PROVIDER_LABELS,
  TProviderId,
  providerUnavailableCopy,
} from "@/features/dashboard/constants/dashboardProviders"
import { ProviderUnavailable } from "@/features/dashboard/presentations/ProviderUnavailable"
import { TPendingAction, type IDashboardSnapshot } from "@/features/dashboard/schemas/dashboard.schema"

interface IProviderPanelProps {
  provider: TProviderId
  children: ReactNode
  snapshot?: IDashboardSnapshot
  pending?: TPendingAction
  onLaunch?: () => Promise<void>
}

export const ProviderPanel: FC<IProviderPanelProps> = ({
  provider,
  children,
  snapshot,
  pending,
  onLaunch,
}) => {
  const providerBody = useMemo(() => {
    if (provider === TProviderId.OpenCode) return children
    const label = PROVIDER_LABELS[provider]
    return <ProviderUnavailable label={label} message={providerUnavailableCopy(label)} />
  }, [children, provider])

  return (
    <main className="app-shell app-stack">
      {providerBody}
      {snapshot && pending && onLaunch && (
        <LaunchOptionsSection
          canLaunch={provider === TProviderId.OpenCode}
          pending={pending}
          snapshot={snapshot}
          onLaunch={onLaunch}
        />
      )}
    </main>
  )
}
