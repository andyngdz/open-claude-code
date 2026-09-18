import { Card } from "@heroui/react"
import { clsx } from "clsx"
import type { FC } from "react"

import { WORKBENCH_CARD_CLASS } from "@/features/dashboard/constants/dashboardLayout"
import {
  connectionStatusLabel,
  modelAvailabilityLabel,
} from "@/features/dashboard/services/dashboardFormatters"
import { TConnectionStatus, type IDashboardSnapshot } from "@/features/dashboard/schemas/dashboard.schema"

interface IStatusStripProps {
  snapshot: IDashboardSnapshot
  providerLabel: string
}

/// Compact facts for the current provider: connection, provider, and catalog size.
export const StatusStrip: FC<IStatusStripProps> = ({ snapshot, providerLabel }) => {
  const isConnected = snapshot.connection.status === TConnectionStatus.Connected

  return (
    <Card className={WORKBENCH_CARD_CLASS}>
      <Card.Content>
        <div
          className={clsx(
            "flex w-full flex-wrap items-center gap-2",
            "text-sm font-semibold text-muted",
            "md:gap-0 md:divide-x md:divide-border",
          )}
        >
          <p
            className={
              isConnected
                ? "inline-flex items-center gap-2 text-success md:px-4 md:first:ps-0"
                : "inline-flex items-center gap-2 md:px-4 md:first:ps-0"
            }
          >
            {isConnected && <span aria-hidden className="size-2 shrink-0 rounded-full bg-success" />}
            {connectionStatusLabel(snapshot)}
          </p>
          <p className="md:px-4">Provider: {providerLabel}</p>
          <p className="md:px-4 md:last:pe-0">Models: {modelAvailabilityLabel(snapshot)}</p>
        </div>
      </Card.Content>
    </Card>
  )
}
