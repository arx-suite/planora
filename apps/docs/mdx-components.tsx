import {
    ScrollArea,
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@planora/ui";
import { AlertCircle, AlertTriangle, CheckCircle, ExternalLink, Info } from "@planora/ui/icons";
import { cn } from "@planora/ui/lib";
import type { MDXComponents } from "mdx/types";
import Link from "next/link";
import type { ComponentPropsWithoutRef, ReactNode } from "react";

type AlertVariant = "info" | "warning" | "success" | "danger" | "tip" | "note";

interface CalloutProps {
    children: React.ReactNode;
    type?: AlertVariant;
    title?: string;
}

const alertConfig: Record<AlertVariant, { icon: ReactNode; styles: string }> = {
    info: {
        icon: <Info className="h-4 w-4" />,
        styles: "border-blue-500/30 bg-blue-500/10 [&_svg]:text-blue-500",
    },
    warning: {
        icon: <AlertTriangle className="h-4 w-4" />,
        styles: "border-yellow-500/30 bg-yellow-500/10 [&_svg]:text-yellow-500",
    },
    success: {
        icon: <CheckCircle className="h-4 w-4" />,
        styles: "border-green-500/30 bg-green-500/10 [&_svg]:text-green-500",
    },
    danger: {
        icon: <AlertCircle className="h-4 w-4" />,
        styles: "border-red-500/30 bg-red-500/10 [&_svg]:text-red-500",
    },
    tip: {
        icon: <CheckCircle className="h-4 w-4" />,
        styles: "border-emerald-500/30 bg-emerald-500/10 [&_svg]:text-emerald-500",
    },
    note: {
        icon: <Info className="h-4 w-4" />,
        styles: "border-slate-500/30 bg-slate-500/10 [&_svg]:text-slate-500",
    },
};

export function Note({ children, title }: Partial<CalloutProps>) {
    return (
        <Callout type="note" title={title}>
            {children}
        </Callout>
    );
}

export function Callout({ children, type = "info", title }: CalloutProps) {
    const config = alertConfig[type];
    const defaultTitle = {
        info: "Information",
        warning: "Warning",
        success: "Success",
        danger: "Error",
        tip: "Tip",
        note: "Note",
    }[type];

    return (
        <div className={cn("my-6 rounded-lg border p-4", config.styles)}>
            <div className="flex gap-3">
                <div className="mt-0.5 shrink-0">{config.icon}</div>
                <div className="flex-1 space-y-2">
                    <div className="font-medium text-foreground">{title || defaultTitle}</div>
                    <div className="text-sm [&_p]:mt-0 [&_p]:mb-2 last:mb-0">{children}</div>
                </div>
            </div>
        </div>
    );
}

type HeadingProps = ComponentPropsWithoutRef<"h1">;
type TextProps = ComponentPropsWithoutRef<"p">;
type AnchorProps = ComponentPropsWithoutRef<"a">;

export const mdxComponents = {
    h1: (props: HeadingProps) => (
        <h1
            className="group scroll-mt-28 text-4xl font-bold tracking-tight mt-10 mb-6"
            {...props}
        />
    ),
    h2: ({ children, ...props }: HeadingProps) => (
        <h2
            className="group scroll-mt-28 text-2xl font-semibold tracking-tight mt-10 mb-4 flex items-center gap-2"
            {...props}
        >
            {children}
            <a
                href={`#${children?.toString().toLowerCase().replace(/\s+/g, "-")}`}
                className="invisible group-hover:visible text-muted-foreground hover:text-foreground"
                aria-label="Link to section"
            >
                #
            </a>
        </h2>
    ),
    h3: (props: HeadingProps) => (
        <h3 className="scroll-mt-28 text-xl font-semibold mt-8 mb-3" {...props} />
    ),

    p: (props: TextProps) => <p className="leading-7 text-muted-foreground mt-4 mb-4" {...props} />,

    ul: (props: ComponentPropsWithoutRef<"ul">) => (
        <ul className="ml-6 list-disc space-y-2 text-muted-foreground mt-4" {...props} />
    ),
    ol: (props: ComponentPropsWithoutRef<"ol">) => (
        <ol className="ml-6 list-decimal space-y-2 text-muted-foreground mt-4" {...props} />
    ),

    a: ({ href, ...props }: AnchorProps) => {
        const className =
            "font-medium text-primary underline underline-offset-4 hover:text-primary/80 transition-colors";
        const isExternal = href?.startsWith("http");

        if (href?.startsWith("/")) {
            return <Link href={href} className={className} {...props} />;
        }

        return (
            <a
                href={href}
                target={isExternal ? "_blank" : undefined}
                rel={isExternal ? "noopener noreferrer" : undefined}
                className={cn(className, isExternal && "inline-flex items-center gap-1")}
                {...props}
            >
                {props.children}
                {isExternal && <ExternalLink className="h-3 w-3" />}
            </a>
        );
    },

    code: ({ children, className }: ComponentPropsWithoutRef<"code">) => {
        const isInline = !className?.includes("language-");

        if (isInline) {
            return (
                <code className="rounded-md bg-muted px-1.5 py-0.5 text-sm font-mono">
                    {children}
                </code>
            );
        }

        return <code className={className}>{children}</code>;
    },

    table: (props: ComponentPropsWithoutRef<"table">) => (
        <div className="my-8 overflow-hidden rounded-lg border">
            <ScrollArea>
                <Table {...props} />
            </ScrollArea>
        </div>
    ),
    thead: TableHeader,
    tbody: TableBody,
    tr: TableRow,
    th: (props: ComponentPropsWithoutRef<"th">) => (
        <TableHead className="font-semibold bg-muted/50" {...props} />
    ),
    td: (props: ComponentPropsWithoutRef<"td">) => (
        <TableCell className="py-3 align-top" {...props} />
    ),

    blockquote: ({ children }: ComponentPropsWithoutRef<"blockquote">) => <Note>{children}</Note>,

    hr: () => <hr className="my-8 border-t border-muted" />,
};

export function useMDXComponents(components: MDXComponents): MDXComponents {
    return {
        ...mdxComponents,
        ...components,
    };
}
