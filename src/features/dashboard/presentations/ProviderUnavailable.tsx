import { Card } from "@heroui/react"
import type { FC } from "react"

import { WORKBENCH_CARD_CLASS } from "@/features/dashboard/constants/dashboardLayout"

interface IProviderUnavailableProps {
  label: string
  message: string
}

export const ProviderUnavailable: FC<IProviderUnavailableProps> = ({ label, message }) => {
  return (
    <Card className={WORKBENCH_CARD_CLASS}>
      <Card.Header className="flex flex-col gap-2">
        <Card.Title className="text-2xl font-bold">{label}</Card.Title>
        <Card.Description>{message}</Card.Description>
      </Card.Header>
    </Card>
  )
}
