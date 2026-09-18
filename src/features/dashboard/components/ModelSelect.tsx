import { Autocomplete, EmptyState, FieldError, Label, ListBox, SearchField, useFilter } from "@heroui/react"
import { isString, map } from "es-toolkit/compat"
import { useController } from "react-hook-form"
import type { FC } from "react"

import {
  MODEL_SEARCH_EMPTY,
  MODEL_SEARCH_PLACEHOLDER,
  type TModelField,
} from "@/features/dashboard/constants/dashboardLabels"
import type { IModelOption } from "@/features/dashboard/schemas/dashboard.schema"

interface IModelSelectProps {
  label: string
  name: TModelField
  options: readonly IModelOption[]
}

export const ModelSelect: FC<IModelSelectProps> = ({ label, name, options }) => {
  const { field, fieldState } = useController({ name })
  const { contains } = useFilter({ sensitivity: "base" })
  const message = fieldState.error?.message

  return (
    <Autocomplete
      fullWidth
      isInvalid={isString(message)}
      name={field.name}
      value={field.value}
      onBlur={field.onBlur}
      onChange={(nextValue) => {
        if (!isString(nextValue)) return
        field.onChange(nextValue)
      }}
    >
      <Label>{label}</Label>
      <Autocomplete.Trigger>
        <Autocomplete.Value />
        <Autocomplete.Indicator />
      </Autocomplete.Trigger>
      <FieldError>{message}</FieldError>
      <Autocomplete.Popover>
        <Autocomplete.Filter filter={contains}>
          <SearchField aria-label={MODEL_SEARCH_PLACEHOLDER} autoFocus variant="secondary">
            <SearchField.Group>
              <SearchField.SearchIcon />
              <SearchField.Input placeholder={MODEL_SEARCH_PLACEHOLDER} />
              <SearchField.ClearButton />
            </SearchField.Group>
          </SearchField>
          <ListBox renderEmptyState={() => <EmptyState>{MODEL_SEARCH_EMPTY}</EmptyState>}>
            {map(options, (option) => (
              <ListBox.Item id={option.id} key={option.id} textValue={option.displayName}>
                {option.displayName}
                <ListBox.ItemIndicator />
              </ListBox.Item>
            ))}
          </ListBox>
        </Autocomplete.Filter>
      </Autocomplete.Popover>
    </Autocomplete>
  )
}
