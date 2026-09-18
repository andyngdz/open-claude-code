import { clsx } from "clsx"
import { useMemo } from "react"
import type { FC, ReactNode } from "react"

import {
  PROVIDER_LABELS,
  TProviderId,
  providerUnavailableCopy,
} from "@/features/dashboard/constants/dashboardProviders"
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

  return (
    <main
      className={clsx(
        "mx-auto flex w-full max-w-7xl flex-col gap-6",
        "p-4 md:p-8",
      )}
    >
      {providerBody}
    </main>
  )
}
