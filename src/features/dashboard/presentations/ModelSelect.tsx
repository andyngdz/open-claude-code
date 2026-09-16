import type { FC } from "react"

import type { IModelOption } from "@/features/dashboard/constants/dashboardSchema"

interface IModelSelectProps {
  id: string
  label: string
  value: string
  options: readonly IModelOption[]
  errorMessage?: string
  onChange: (modelId: string) => void
}

export const ModelSelect: FC<IModelSelectProps> = ({
  id,
  label,
  value,
  options,
  errorMessage,
  onChange,
}) => {
  return (
    <label className="field" htmlFor={id}>
      <span>{label}</span>
      <select id={id} value={value} onChange={(event) => onChange(event.target.value)}>
        {options.map((option) => (
          <option key={option.id} value={option.id}>
            {option.displayName}
          </option>
        ))}
      </select>
      {errorMessage && <span role="alert">{errorMessage}</span>}
    </label>
  )
}
