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
    <Card className="launch-card workbench-card w-full">
      <Card.Content className="workbench-card-content">
        <div className="flex flex-col gap-4">
          <LaunchCommandField copied={copied} onCopy={onCopy} />
          <Button
            className="launch-card-action"
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
      </Card.Content>
    </Card>
  )
}
