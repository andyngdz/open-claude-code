import { Card } from "@heroui/react"
import type { FC } from "react"

interface IProviderUnavailableProps {
  label: string
  message: string
}

export const ProviderUnavailable: FC<IProviderUnavailableProps> = ({ label, message }) => {
  return (
    <Card className="w-full">
      <Card.Header>
        <Card.Title>{label}</Card.Title>
        <Card.Description>{message}</Card.Description>
      </Card.Header>
    </Card>
  )
}
