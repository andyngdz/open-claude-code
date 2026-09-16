import { Button, Input, TextField } from "@heroui/react"
import { useController } from "react-hook-form"
import type { FC } from "react"

import type { ValueChanged } from "@/types"

interface ICustomModelRowProps {
  index: number
  isDisabled: boolean
  onRemove: ValueChanged<number>
}

export const CustomModelRow: FC<ICustomModelRowProps> = ({ index, isDisabled, onRemove }) => {
  const { field } = useController({ name: `customModels.${index}.modelId` })

  return (
    <div className="flex items-center gap-2">
      <div className="min-w-0 flex-1">
        <TextField
          aria-label="Custom model"
          fullWidth
          isDisabled={isDisabled}
          name={field.name}
          value={field.value}
          onBlur={field.onBlur}
          onChange={field.onChange}
        >
          <Input ref={field.ref} />
        </TextField>
      </div>
      <Button
        isDisabled={isDisabled}
        type="button"
        variant="secondary"
        onPress={() => {
          onRemove(index)
        }}
      >
        Remove
      </Button>
    </div>
  )
}
