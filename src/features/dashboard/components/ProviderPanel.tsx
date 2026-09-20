import { useMemo } from "react"
import type { FC, ReactNode } from "react"

import {
  PROVIDER_LABELS,
  TProviderId,
  providerUnavailableCopy,
} from "@/features/dashboard/constants/dashboardProviders"
import { WORKBENCH_PANEL_CLASS } from "@/features/dashboard/constants/dashboardLayout"
import { ProviderUnavailable } from "@/features/dashboard/presentations/ProviderUnavailable"

interface IProviderPanelProps {
  provider: TProviderId
  children: ReactNode
}

export const ProviderPanel: FC<IProviderPanelProps> = ({
  provider,
  children,
}) => {
  const providerBody = useMemo(() => {
    if (provider === TProviderId.OpenCode) return children
    const label = PROVIDER_LABELS[provider]
    return <ProviderUnavailable label={label} message={providerUnavailableCopy(label)} />
  }, [children, provider])

  return <main className={WORKBENCH_PANEL_CLASS}>{providerBody}</main>
}
