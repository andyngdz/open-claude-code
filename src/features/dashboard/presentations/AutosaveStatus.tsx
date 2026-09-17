import { Chip } from "@heroui/react"
import type { FC } from "react"

import {
  AUTOSAVE_STATUS_LABELS,
  TAutosaveStatus,
} from "@/features/dashboard/constants/dashboardLabels"

interface IAutosaveStatusProps {
  status: TAutosaveStatus
}

/// Compact persist fact next to Models. Hidden until a save is in flight.
export const AutosaveStatus: FC<IAutosaveStatusProps> = ({ status }) => {
  if (status === TAutosaveStatus.Idle) return
  if (status === TAutosaveStatus.Saved) {
    return (
      <Chip color="success" variant="tertiary">
        {AUTOSAVE_STATUS_LABELS[status]}
      </Chip>
    )
  }
  if (status === TAutosaveStatus.Failed) {
    return (
      <Chip color="danger" variant="tertiary">
        {AUTOSAVE_STATUS_LABELS[status]}
      </Chip>
    )
  }
  return (
    <Chip color="accent" variant="tertiary">
      {AUTOSAVE_STATUS_LABELS[status]}
    </Chip>
  )
}
