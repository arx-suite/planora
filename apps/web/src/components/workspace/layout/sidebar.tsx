"use client";

import {
    Collapsible,
    CollapsibleContent,
    CollapsibleTrigger,
    Sidebar,
    SidebarContent,
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from "@planora/ui";
import {
    ArrowRight,
    Brain,
    ChevronDown,
    Clock,
    Home,
    Orbit,
    Settings,
    SquareDashedKanban,
} from "lucide-react";
import Image from "next/image";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { useWorkspace } from "@/context/workspace-provider";
import { config } from "@/lib/config";
import { cn } from "@/lib/utils";

const recentSpaces = [
    { id: "1", name: "Engineering", href: "/spaces/engineering" },
    { id: "2", name: "Marketing", href: "/spaces/marketing" },
];

const recentProjects = [
    { id: "1", name: "API", href: "/projects/api" },
    { id: "2", name: "Frontend", href: "/projects/frontend" },
];

const recentActivity = [
    {
        id: "1",
        label: "Engineering / API",
        href: "/spaces/engineering/projects/api",
    },
    {
        id: "2",
        label: "Marketing / Website",
        href: "/spaces/marketing/projects/website",
    },
];

export function WorkspaceNavbar() {
    const pathname = usePathname();
    const { info } = useWorkspace();

    return (
        <Sidebar>
            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupLabel>
                        <Link
                            href={config.nextjs}
                            className="flex items-center gap-2 text-xl font-semibold tracking-tight"
                        >
                            <Image width={36} height={36} alt="Planora" src="/planora.png" />
                            <span className="bg-linear-to-r from-indigo-500 to-violet-500 bg-clip-text text-transparent">
                                Planora
                            </span>
                        </Link>
                    </SidebarGroupLabel>

                    <SidebarGroupContent className="mt-6">
                        <SidebarMenu>
                            <NavItem
                                icon={Home}
                                label="Dashboard"
                                href="/dashboard"
                                active={pathname.startsWith("/dashboard")}
                            />

                            {info.spaceEnabled && (
                                <NavDropdown
                                    icon={Orbit}
                                    label="Spaces"
                                    items={recentSpaces}
                                    viewAllHref="/spaces"
                                    active={pathname.startsWith("/spaces")}
                                />
                            )}

                            {!info.spaceEnabled && (
                                <NavDropdown
                                    icon={SquareDashedKanban}
                                    label="Projects"
                                    items={recentProjects}
                                    viewAllHref="/projects"
                                    active={pathname.startsWith("/projects")}
                                />
                            )}

                            <NavItem
                                icon={Brain}
                                label="Insight"
                                href="/insight"
                                active={pathname.startsWith("/insight")}
                            />

                            <NavDropdown
                                icon={Clock}
                                label="Recent"
                                items={recentActivity}
                                viewAllHref="/recent"
                            />

                            <NavItem
                                icon={Settings}
                                label="Settings"
                                href="/settings"
                                active={pathname.startsWith("/settings")}
                            />
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>
        </Sidebar>
    );
}

function NavItem({
    icon: Icon,
    label,
    href,
    active,
}: {
    icon: React.ElementType;
    label: string;
    href: string;
    active?: boolean;
}) {
    return (
        <SidebarMenuItem>
            <SidebarMenuButton asChild isActive={active}>
                <Link href={href} className="flex items-center gap-2">
                    <Icon className="h-4 w-4" />
                    <span>{label}</span>
                </Link>
            </SidebarMenuButton>
        </SidebarMenuItem>
    );
}

function NavDropdown({
    icon: Icon,
    label,
    items,
    viewAllHref,
    active,
}: {
    icon: React.ElementType;
    label: string;
    items: { id: string; name?: string; label?: string; href: string }[];
    viewAllHref: string;
    active?: boolean;
}) {
    return (
        <SidebarMenuItem>
            <Collapsible defaultOpen={active}>
                <CollapsibleTrigger asChild>
                    <SidebarMenuButton className="flex items-center justify-between">
                        <div className="flex items-center gap-2">
                            <Icon className="h-4 w-4" />
                            <span>{label}</span>
                        </div>
                        <ChevronDown className="h-4 w-4 opacity-60" />
                    </SidebarMenuButton>
                </CollapsibleTrigger>

                <CollapsibleContent>
                    <div className="ml-6 mt-1 space-y-1">
                        {items.map((item) => (
                            <Link
                                key={item.id}
                                href={item.href}
                                className={cn(
                                    "block rounded-md px-2 py-1 text-sm text-muted-foreground",
                                    "hover:bg-muted hover:text-foreground transition",
                                )}
                            >
                                {item.name ?? item.label}
                            </Link>
                        ))}

                        <Link
                            href={viewAllHref}
                            className="block px-2 py-1 text-xs text-muted-foreground hover:text-foreground"
                        >
                            <span className="flex gap-2 items-center">
                                View all <ArrowRight className="w-4 h-4" />
                            </span>
                        </Link>
                    </div>
                </CollapsibleContent>
            </Collapsible>
        </SidebarMenuItem>
    );
}
