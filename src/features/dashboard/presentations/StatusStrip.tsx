import { Card } from "@heroui/react"
import type { FC } from "react"

import {
  connectionStatusLabel,
  isLaunchReady,
  modelAvailabilityLabel,
} from "@/features/dashboard/services/dashboardFormatters"
import { TConnectionStatus, type IDashboardSnapshot } from "@/features/dashboard/schemas/dashboard.schema"

interface IStatusStripProps {
  snapshot: IDashboardSnapshot
  providerLabel: string
}

/// Compact facts for the current provider: connection, catalog, gateway, and launch readiness.
export const StatusStrip: FC<IStatusStripProps> = ({ snapshot, providerLabel }) => {
  const isConnected = snapshot.connection.status === TConnectionStatus.Connected
  const launchReady = isLaunchReady(snapshot)

  return (
    <Card className="status-strip">
      <Card.Content>
        <div className="status-strip-row">
          <p className={isConnected ? "status-strip-item status-strip-ready" : "status-strip-item"}>
            {connectionStatusLabel(snapshot)}
          </p>
          <p className="status-strip-item">Provider: {providerLabel}</p>
          <p className="status-strip-item">Models: {modelAvailabilityLabel(snapshot)}</p>
          <p className="status-strip-item">Local gateway: {snapshot.gatewayBaseUrl}</p>
          <p className={launchReady ? "status-strip-item status-strip-ready" : "status-strip-item"}>
            {launchReady ? "Ready to launch" : "Not ready"}
          </p>
        </div>
      </Card.Content>
    </Card>
  )
}
