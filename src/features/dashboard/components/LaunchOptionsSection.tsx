import { Card } from "@heroui/react"
import type { FC } from "react"

import { LoadingButton } from "@/common/components/LoadingButton"
import { WORKBENCH_CARD_CLASS } from "@/features/dashboard/constants/dashboardLayout"
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
    <>
      <Card className={WORKBENCH_CARD_CLASS}>
        <Card.Content>
          <LaunchCommandField copied={copied} onCopy={onCopy} />
        </Card.Content>
      </Card>
      <Card className={WORKBENCH_CARD_CLASS}>
        <Card.Content>
          <LoadingButton
            className="rounded-md"
            fullWidth
            isDisabled={!canPressLaunch}
            isPending={isLaunching}
            size="lg"
            type="button"
            onPress={() => {
              void onPressLaunch()
            }}
          >
            Launch Claude Code
          </LoadingButton>
        </Card.Content>
      </Card>
    </>
  )
}
