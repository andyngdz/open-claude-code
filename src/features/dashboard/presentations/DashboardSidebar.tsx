import { ListBox, Separator } from "@heroui/react"
import { clsx } from "clsx"
import { isString, map } from "es-toolkit/compat"
import { Box, CodeXml, Hexagon, Server, Settings, Sparkles } from "lucide-react"
import type { FC, ReactNode } from "react"

import {
  PROVIDER_LABELS,
  PROVIDERS,
  TProviderId,
} from "@/features/dashboard/constants/dashboardProviders"
import { SETTINGS_TITLE } from "@/features/dashboard/constants/dashboardLabels"
import {
  TSidebarView,
  isSettingsView,
  type TSidebarSelection,
} from "@/features/dashboard/constants/dashboardNavigation"
import { SidebarNavItem } from "@/features/dashboard/presentations/SidebarNavItem"
import { readProviderId } from "@/features/dashboard/services/dashboardFormatters"
import type { ValueChanged } from "@/types"

/// Each provider row carries its own mark, so a provider added without one
/// fails the type check instead of rendering a blank row.
const PROVIDER_ICONS = {
  [TProviderId.OpenCode]: <Server size={16} />,
  [TProviderId.Codex]: <CodeXml size={16} />,
  [TProviderId.Grok]: <Hexagon size={16} />,
  [TProviderId.Cursor]: <Box size={16} />,
  [TProviderId.Gemini]: <Sparkles size={16} />,
} satisfies Record<TProviderId, ReactNode>

interface IDashboardSidebarProps {
  selected: TSidebarSelection
  onSelect: ValueChanged<TSidebarSelection>
}

export const DashboardSidebar: FC<IDashboardSidebarProps> = ({
  selected,
  onSelect,
}) => {
  const settingsOpen = isSettingsView(selected)
  const providerKeys = new Set<string>(settingsOpen ? [] : [selected])
  const settingsKeys = new Set<string>(settingsOpen ? [selected] : [])

  return (
    <aside
      className={clsx(
        "flex w-full shrink-0 flex-col justify-between gap-4",
        "border-b border-border bg-background-secondary",
        "px-4 py-6 md:w-80",
        "md:border-r md:border-b-0",
      )}
    >
      <div className="flex flex-col gap-4">
        <div className="flex flex-col gap-2 px-4">
          <p className="whitespace-nowrap text-2xl font-bold text-foreground">Open Claude Code</p>
          <p className="text-sm text-muted">Your local gateway for Claude Code.</p>
        </div>
        <p className="px-4 text-sm font-semibold text-muted">Providers</p>
        <ListBox
          aria-label="Providers"
          className="w-full"
          selectedKeys={providerKeys}
          selectionMode="single"
          onSelectionChange={(keys) => {
            if (keys === "all") return
            const [key] = keys
            if (!isString(key)) return
            const provider = readProviderId(key)
            if (!provider) return
            onSelect(provider)
          }}
        >
          {map(PROVIDERS, (provider) => (
            <SidebarNavItem
              icon={PROVIDER_ICONS[provider]}
              id={provider}
              key={provider}
              label={PROVIDER_LABELS[provider]}
            />
          ))}
        </ListBox>
      </div>
      <div className="flex flex-col gap-4">
        <Separator />
        <ListBox
          aria-label={SETTINGS_TITLE}
          className="w-full"
          selectedKeys={settingsKeys}
          selectionMode="single"
          onSelectionChange={(keys) => {
            if (keys === "all") return
            const [key] = keys
            if (key !== TSidebarView.Settings) return
            onSelect(TSidebarView.Settings)
          }}
        >
          <SidebarNavItem
            icon={<Settings size={16} />}
            id={TSidebarView.Settings}
            label={SETTINGS_TITLE}
          />
        </ListBox>
      </div>
    </aside>
  )
}
