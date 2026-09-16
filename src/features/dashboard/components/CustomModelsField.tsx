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
      <p className="text-sm font-medium">Custom models</p>
      {map(fields, (field, index) => (
        <CustomModelRow index={index} isDisabled={isDisabled} key={field.id} onRemove={remove} />
      ))}
      <div>
        <Button
          aria-label="Add custom model"
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
    </div>
  )
}
