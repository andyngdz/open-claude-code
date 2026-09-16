import { map } from "es-toolkit/compat"
import { useController } from "react-hook-form"
import type { FC } from "react"

import { FieldSelect } from "@/features/dashboard/components/FieldSelect"
import {
  terminalKindSchema,
  type IDashboardSnapshot,
} from "@/features/dashboard/schemas/dashboard.schema"

interface ITerminalSelectProps {
  terminals: IDashboardSnapshot["terminals"]
}

export const TerminalSelect: FC<ITerminalSelectProps> = ({ terminals }) => {
  const { field, fieldState } = useController({ name: "terminal" })

  return (
    <FieldSelect
      label="Terminal"
      message={fieldState.error?.message}
      name={field.name}
      options={map(terminals, (terminal) => {
        return {
          id: terminal.kind,
          label: terminal.label,
          isDisabled: !terminal.isAvailable,
        }
      })}
      value={field.value}
      onBlur={field.onBlur}
      onChange={(value) => {
        const parsed = terminalKindSchema.safeParse(value)
        if (!parsed.success) return
        field.onChange(parsed.data)
      }}
    />
  )
}
