import { Checkbox } from "@heroui/react"
import { useController } from "react-hook-form"
import type { FC } from "react"

import {
  EXTENDED_CONTEXT_LABEL,
  type TExtendedField,
} from "@/features/dashboard/constants/dashboardLabels"

interface IOneMillionCheckboxProps {
  /// Names the row in the accessible label, so four ticks on screen read apart.
  aliasLabel: string
  name: TExtendedField
}

export const OneMillionCheckbox: FC<IOneMillionCheckboxProps> = ({ aliasLabel, name }) => {
  const { field } = useController({ name })

  return (
    <Checkbox
      aria-label={`${aliasLabel} ${EXTENDED_CONTEXT_LABEL}`}
      isSelected={field.value}
      name={field.name}
      onBlur={field.onBlur}
      onChange={field.onChange}
    >
      <Checkbox.Content>
        <Checkbox.Control>
          <Checkbox.Indicator />
        </Checkbox.Control>
        {EXTENDED_CONTEXT_LABEL}
      </Checkbox.Content>
    </Checkbox>
  )
}
