import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";
import type * as React from "react";

import { cn } from "../../lib/utils";

export type ButtonVariantsProps = VariantProps<typeof buttonVariants>;

const buttonVariants = cva(
    `relative 
    flex items-center justify-center
    cursor-pointer 
    inline-flex 
    items-center 
    space-x-2 
    text-center 
    font-regular 
    ease-out 
    duration-200 
    rounded-md
    outline-none 
    transition-all 
    outline-0 
    focus-visible:outline-4 
    focus-visible:outline-offset-1
    border
    `,
    {
        variants: {
            variant: {
                primary: `
                    bg-brand-400 dark:bg-brand-500 
                    hover:bg-brand/80 dark:hover:bg-brand/50
                    text-foreground
                    outline-solid
                    border-brand-500/75 dark:border-brand/30
                    hover:border-brand-600 dark:hover:border-brand
                    focus-visible:outline-brand-600
                    data-[state=open]:bg-brand-400/80 dark:data-[state=open]:bg-brand-500/80
                    data-[state=open]:outline-brand-600
                `,
                default: `
                    text-foreground
                    bg-muted  hover:bg-selection
                    border-strong hover:border-stronger
                    focus-visible:outline-brand-600
                    data-[state=open]:bg-selection
                    data-[state=open]:outline-brand-600
                    data-[state=open]:border-button-hover
                `,
                destructive:
                    "bg-destructive text-white hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60",
                outline:
                    "border border-primary bg-background shadow-xs hover:bg-primary/50 hover:text-accent-foreground dark:bg-primary/30 dark:border-input dark:hover:bg-primary/50",
                secondary: "bg-brand-secondary text-secondary-foreground hover:bg-secondary/80",
                ghost: "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50",
                link: "text-primary underline-offset-4 hover:underline border-0",
            },
            size: {
                default: "h-9 px-4 py-2 has-[>svg]:px-3",
                sm: "h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5",
                lg: "h-10 rounded-md px-6 has-[>svg]:px-4",
                icon: "size-9",
                "icon-sm": "size-8",
                "icon-lg": "size-10",
            },
        },
        defaultVariants: {
            variant: "default",
            size: "default",
        },
    },
);

function Button({
    className,
    variant = "default",
    size = "default",
    asChild = false,
    ...props
}: React.ComponentProps<"button"> &
    VariantProps<typeof buttonVariants> & {
        asChild?: boolean;
    }) {
    const Comp = asChild ? Slot : "button";

    return (
        <Comp
            data-slot="button"
            data-variant={variant}
            data-size={size}
            className={cn(buttonVariants({ variant, size, className }))}
            {...props}
        />
    );
}

export { Button, buttonVariants };
