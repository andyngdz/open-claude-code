import { Card, Switch } from "@heroui/react"
import type { FC } from "react"

import { WORKBENCH_CARD_CLASS } from "@/features/dashboard/constants/dashboardLayout"
import {
  STARTUP_HINT,
  STARTUP_LABEL,
  STARTUP_TITLE,
} from "@/features/dashboard/constants/dashboardLabels"
import { useStartupSection } from "@/features/dashboard/hooks/useStartupSection"

export const StartupSection: FC = () => {
  const { autoStartEnabled, isBusy, toggle } = useStartupSection()

  return (
    <Card className={WORKBENCH_CARD_CLASS}>
      <Card.Header className="flex flex-col gap-2">
        <Card.Title className="text-2xl font-bold">{STARTUP_TITLE}</Card.Title>
        <Card.Description>{STARTUP_HINT}</Card.Description>
      </Card.Header>
      <Card.Content>
        <Switch isDisabled={isBusy} isSelected={autoStartEnabled} onChange={toggle}>
          <Switch.Content>
            <Switch.Control>
              <Switch.Thumb />
            </Switch.Control>
            {STARTUP_LABEL}
          </Switch.Content>
        </Switch>
      </Card.Content>
    </Card>
  )
}
