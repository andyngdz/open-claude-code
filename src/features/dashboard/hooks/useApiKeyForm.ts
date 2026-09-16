import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"

import { apiKeySchema, type IApiKeyForm } from "@/features/dashboard/constants/dashboardSchema"

const apiKeyDefaults = { apiKey: "" } satisfies IApiKeyForm

export const useApiKeyForm = () => {
  return useForm<IApiKeyForm>({
    resolver: zodResolver(apiKeySchema),
    defaultValues: apiKeyDefaults,
  })
}
