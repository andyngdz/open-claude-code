import { FieldError, Label, ListBox, Select } from "@heroui/react"
import { isString, map } from "es-toolkit/compat"
import type { FC } from "react"

import type { ValueChanged } from "@/types"

interface IFieldSelectOption {
  id: string
  label: string
  isDisabled?: boolean
}

interface IFieldSelectProps {
  label: string
  name: string
  value: string
  message?: string
  options: readonly IFieldSelectOption[]
  onBlur: () => void
  onChange: ValueChanged<string>
}

export const FieldSelect: FC<IFieldSelectProps> = ({
  label,
  name,
  value,
  message,
  options,
  onBlur,
  onChange,
}) => {
  return (
    <Select
      fullWidth
      isInvalid={isString(message)}
      name={name}
      value={value}
      onBlur={onBlur}
      onChange={(nextValue) => {
        if (!isString(nextValue)) return
        onChange(nextValue)
      }}
    >
      <Label>{label}</Label>
      <Select.Trigger>
        <Select.Value />
        <Select.Indicator />
      </Select.Trigger>
      <FieldError>{message}</FieldError>
      <Select.Popover>
        <ListBox>
          {map(options, (option) => (
            <ListBox.Item
              id={option.id}
              isDisabled={option.isDisabled}
              key={option.id}
              textValue={option.label}
            >
              {option.label}
              <ListBox.ItemIndicator />
            </ListBox.Item>
          ))}
        </ListBox>
      </Select.Popover>
    </Select>
  )
}
