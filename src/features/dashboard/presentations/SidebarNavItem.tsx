import { Label, ListBox } from "@heroui/react"
import { clsx } from "clsx"
import type { FC, ReactNode } from "react"

interface ISidebarNavItemProps {
  icon: ReactNode
  id: string
  label: string
}

/// One sidebar row, for any group the sidebar holds.
///
/// The ListBox owns selection, so this only gives the row its mark, its label,
/// and the rest and selected colors that mean "this is the open panel".
export const SidebarNavItem: FC<ISidebarNavItemProps> = ({
  icon,
  id,
  label,
}) => {
  return (
    <ListBox.Item
      className={clsx(
        "rounded-lg py-2 text-muted",
        "aria-selected:bg-accent aria-selected:text-accent-foreground",
        "data-[selected=true]:bg-accent data-[selected=true]:text-accent-foreground",
      )}
      id={id}
      textValue={label}
    >
      <span className="flex items-center gap-2">
        {icon}
        <Label>{label}</Label>
      </span>
    </ListBox.Item>
  )
}
