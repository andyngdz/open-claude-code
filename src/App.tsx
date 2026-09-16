import { Toast } from "@heroui/react"

import { Dashboard } from "@/features/dashboard/components/Dashboard"

export const App = () => {
  return (
    <>
      <Dashboard />
      <Toast.Provider placement="bottom end" />
    </>
  )
}
