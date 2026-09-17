import { Toast } from "@heroui/react"

import { Dashboard } from "@/features/dashboard/components/Dashboard"

export const App = () => {
  return (
    <div className="min-h-dvh bg-background">
      <Dashboard />
      <Toast.Provider placement="bottom end" />
    </div>
  )
}
