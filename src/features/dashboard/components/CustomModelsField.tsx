import { Button } from "@heroui/react"
import { map } from "es-toolkit/compat"
import type { FC } from "react"

import { CustomModelRow } from "@/features/dashboard/components/CustomModelRow"
import { useCustomModelsField } from "@/features/dashboard/hooks/useCustomModelsField"

interface ICustomModelsFieldProps {
  isDisabled: boolean
}

export const CustomModelsField: FC<ICustomModelsFieldProps> = ({ isDisabled }) => {
  const { fields, append, remove } = useCustomModelsField()

  return (
    <div className="flex flex-col gap-2">
      <div>
        <p className="text-sm font-semibold text-foreground">Custom models</p>
        <p className="text-sm text-muted">Add provider model IDs not returned by the catalog.</p>
      </div>
      {map(fields, (field, index) => (
        <CustomModelRow index={index} isDisabled={isDisabled} key={field.id} onRemove={remove} />
      ))}
      <Button
        aria-label="Add custom model"
        className="rounded-md"
        isDisabled={isDisabled}
        isIconOnly
        type="button"
        onPress={() => {
          append({ modelId: "" })
        }}
      >
        +
      </Button>
    </div>
  )
}
