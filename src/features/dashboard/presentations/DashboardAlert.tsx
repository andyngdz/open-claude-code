import { Alert } from "@heroui/react"
import type { FC } from "react"

import { TDashboardAlertStatus } from "@/features/dashboard/constants/dashboardLabels"

interface IDashboardAlertProps {
  message: string
  status: TDashboardAlertStatus
}

export const DashboardAlert: FC<IDashboardAlertProps> = ({ message, status }) => {
  return (
    <Alert role={status === TDashboardAlertStatus.Danger ? "alert" : "status"} status={status}>
      <Alert.Indicator />
      <Alert.Content>
        <Alert.Description>{message}</Alert.Description>
      </Alert.Content>
    </Alert>
  )
}
