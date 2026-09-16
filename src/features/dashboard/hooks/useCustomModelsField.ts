import { useFieldArray, useFormContext } from "react-hook-form"
import type { UseFieldArrayReturn } from "react-hook-form"

import type { ILaunchForm } from "@/features/dashboard/schemas/dashboard.schema"

interface IUseCustomModelsFieldReturn
  extends Pick<UseFieldArrayReturn<ILaunchForm, "customModels">, "fields" | "append" | "remove"> {}

export const useCustomModelsField = () => {
  const { control } = useFormContext<ILaunchForm>()
  const { fields, append, remove } = useFieldArray({ control, name: "customModels" })

  return { fields, append, remove } satisfies IUseCustomModelsFieldReturn
}
