import { Button, Card } from "@heroui/react"
import type { FC } from "react"

import { useLaunchOptionsSection } from "@/features/dashboard/hooks/useLaunchOptionsSection"
import { LaunchCommandField } from "@/features/dashboard/presentations/LaunchCommandField"
import { TPendingAction, type IDashboardSnapshot } from "@/features/dashboard/schemas/dashboard.schema"

interface ILaunchOptionsSectionProps {
  snapshot: IDashboardSnapshot
  pending: TPendingAction
  canLaunch: boolean
  onLaunch: () => Promise<void>
}

export const LaunchOptionsSection: FC<ILaunchOptionsSectionProps> = ({
  snapshot,
  pending,
  canLaunch,
  onLaunch,
}) => {
  const { canPressLaunch, copied, isLaunching, onCopy, onPressLaunch } = useLaunchOptionsSection(
    snapshot,
    pending,
    canLaunch,
    onLaunch,
  )

  return (
    <Card className="w-full">
      <Card.Header>
        <Card.Title>Launch options</Card.Title>
      </Card.Header>
      <Card.Content>
        <div className="flex flex-col gap-4">
          <div>
            <Button
              isDisabled={!canPressLaunch}
              isPending={isLaunching}
              type="button"
              onPress={() => {
                void onPressLaunch()
              }}
            >
              {isLaunching ? "Opening" : "Launch Claude Code"}
            </Button>
          </div>
          <LaunchCommandField copied={copied} onCopy={onCopy} />
        </div>
      </Card.Content>
    </Card>
  )
}
