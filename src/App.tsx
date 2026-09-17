import { Toast } from "@heroui/react"

import { Dashboard } from "@/features/dashboard/components/Dashboard"

export const App = () => {
  return (
    <div className="app-theme">
      <Dashboard />
      <Toast.Provider placement="bottom end" />
    </div>
  )
}
