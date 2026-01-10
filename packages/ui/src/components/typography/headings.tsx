import type { HTMLAttributes } from "react";
import { cn } from "../../lib/utils";

type Props = HTMLAttributes<HTMLElement>;

export function H1({ className, ...props }: Props) {
    return (
        <h1
            className={cn(
                "scroll-m-20 text-2xl font-semibold tracking-tight md:text-4xl",
                className,
            )}
            {...props}
        />
    );
}

export function H2({ className, ...props }: Props) {
    return (
        <h2
            className={cn(
                "scroll-m-20 text-xl font-semibold tracking-tight md:text-3xl",
                className,
            )}
            {...props}
        />
    );
}

export function H3({ className, ...props }: Props) {
    return (
        <h3 className={cn("scroll-m-20 text-lg font-semibold md:text-2xl", className)} {...props} />
    );
}

export function H4({ className, ...props }: Props) {
    return (
        <h4 className={cn("scroll-m-20 text-base font-medium md:text-xl", className)} {...props} />
    );
}
