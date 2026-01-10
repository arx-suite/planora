import type { HTMLAttributes } from "react";
import { cn } from "../../lib/utils";

type Props = HTMLAttributes<HTMLElement>;

export function Text({ className, ...props }: Props) {
    return (
        <p
            className={cn("text-sm leading-relaxed text-foreground md:text-base", className)}
            {...props}
        />
    );
}

export function MutedText({ className, ...props }: Props) {
    return <p className={cn("text-xs text-muted-foreground md:text-sm", className)} {...props} />;
}

export function SmallText({ className, ...props }: Props) {
    return <span className={cn("text-xs text-muted-foreground", className)} {...props} />;
}
