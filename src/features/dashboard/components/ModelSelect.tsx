import { map } from "es-toolkit/compat"
import { useController } from "react-hook-form"
import type { FC } from "react"

import { FieldSelect } from "@/features/dashboard/components/FieldSelect"
import type { TModelField } from "@/features/dashboard/constants/dashboardLabels"
import type { IModelOption } from "@/features/dashboard/schemas/dashboard.schema"

interface IModelSelectProps {
  label: string
  name: TModelField
  options: readonly IModelOption[]
}

export const ModelSelect: FC<IModelSelectProps> = ({ label, name, options }) => {
  const { field, fieldState } = useController({ name })

  return (
    <FieldSelect
      label={label}
      message={fieldState.error?.message}
      name={field.name}
      options={map(options, (option) => {
        return { id: option.id, label: option.displayName }
      })}
      value={field.value}
      onBlur={field.onBlur}
      onChange={field.onChange}
    />
  )
}
