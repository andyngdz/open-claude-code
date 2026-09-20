import type { FC } from "react"

import { StartupSection } from "@/features/dashboard/components/StartupSection"
import { WORKBENCH_PANEL_CLASS } from "@/features/dashboard/constants/dashboardLayout"
import { SETTINGS_TITLE } from "@/features/dashboard/constants/dashboardLabels"

/// The workspace the sidebar's Settings entry opens. It carries the cards that
/// configure the app itself, with no provider in scope.
export const SettingsPanel: FC = () => {
  return (
    <main className={WORKBENCH_PANEL_CLASS}>
      <header className="py-2">
        <h1 className="text-4xl font-bold leading-tight text-foreground">{SETTINGS_TITLE}</h1>
      </header>
      <StartupSection />
    </main>
  )
}
