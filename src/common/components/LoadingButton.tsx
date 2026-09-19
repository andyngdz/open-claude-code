import { Button, Spinner, type ButtonProps } from "@heroui/react"
import type { FC, ReactNode } from "react"

interface ILoadingButtonProps extends Omit<ButtonProps, "children"> {
  children: ReactNode
}

/// HeroUI Button that shows a Spinner while `isPending` and keeps a fixed label.
export const LoadingButton: FC<ILoadingButtonProps> = ({ children, isPending, ...props }) => {
  return (
    <Button {...props} isPending={isPending}>
      {({ isPending: pending }) => (
        <>
          {pending && <Spinner color="current" size="sm" />}
          {children}
        </>
      )}
    </Button>
  )
}
