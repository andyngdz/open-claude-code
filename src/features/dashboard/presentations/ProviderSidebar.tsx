import { Label, ListBox } from "@heroui/react"
import { clsx } from "clsx"
import { isString, map } from "es-toolkit/compat"
import { Box, CodeXml, Hexagon, Server, Sparkles } from "lucide-react"
import type { FC } from "react"

import { PROVIDER_LABELS, PROVIDERS, TProviderId } from "@/features/dashboard/constants/dashboardProviders"
import { readProviderId } from "@/features/dashboard/services/dashboardFormatters"
import type { ValueChanged } from "@/types"

interface IProviderSidebarProps {
  selected: TProviderId
  onSelect: ValueChanged<TProviderId>
}

export const ProviderSidebar: FC<IProviderSidebarProps> = ({ selected, onSelect }) => {
  return (
    <aside
      className={clsx(
        "flex w-full shrink-0 flex-col gap-4",
        "border-b border-border bg-background-secondary",
        "px-4 py-6 md:w-80",
        "md:border-r md:border-b-0",
      )}
    >
      <div className="flex flex-col gap-2 px-4">
        <p className="whitespace-nowrap text-2xl font-bold text-foreground">Open Claude Code</p>
        <p className="text-sm text-muted">Your local gateway for Claude Code.</p>
      </div>
      <p className="px-4 text-sm font-semibold text-muted">Providers</p>
      <ListBox
        aria-label="Providers"
        className="w-full"
        selectedKeys={new Set([selected])}
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
          <ListBox.Item
            className={clsx(
              "rounded-lg py-2 text-muted",
              "aria-selected:bg-accent aria-selected:text-accent-foreground",
              "data-[selected=true]:bg-accent data-[selected=true]:text-accent-foreground",
            )}
            id={provider}
            key={provider}
            textValue={PROVIDER_LABELS[provider]}
          >
            <span className="flex items-center gap-2">
              {provider === TProviderId.OpenCode && <Server size={16} />}
              {provider === TProviderId.Codex && <CodeXml size={16} />}
              {provider === TProviderId.Grok && <Hexagon size={16} />}
              {provider === TProviderId.Cursor && <Box size={16} />}
              {provider === TProviderId.Gemini && <Sparkles size={16} />}
              <Label>{PROVIDER_LABELS[provider]}</Label>
            </span>
          </ListBox.Item>
        ))}
      </ListBox>
    </aside>
  )
}
