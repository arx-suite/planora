import { Home, Orbit, Settings, SquareDashedKanban } from "lucide-react";
import Image from "next/image";
import Link from "next/link";
import {
    Sidebar,
    SidebarContent,
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from "@/components/ui/sidebar";
import { config } from "@/lib/config";

const items = [
    {
        title: "Dashboard",
        url: "/dashboard",
        icon: Home,
    },
    {
        title: "Spaces",
        url: "/spaces",
        icon: Orbit,
    },
    {
        title: "Projects",
        url: "/projects",
        icon: SquareDashedKanban,
    },
    {
        title: "Settings",
        url: "/settings",
        icon: Settings,
    },
    /*
    {
        title: "Chat",
        url: "/chat",
        icon: MessageCircle,
    },
    */
];

export function WorkspaceNavbar() {
    return (
        <Sidebar>
            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupLabel>
                        <Link
                            href={config.nextjs}
                            className="text-xl font-semibold tracking-tight flex items-center gap-1"
                        >
                            <Image
                                width={40}
                                height={40}
                                alt="Planora"
                                src="/planora.png"
                            />
                            <span className="bg-linear-to-r from-indigo-500 to-violet-500 bg-clip-text text-transparent">
                                Planora
                            </span>
                        </Link>
                    </SidebarGroupLabel>
                    <SidebarGroupContent className="mt-5">
                        <SidebarMenu>
                            {items.map((item) => (
                                <SidebarMenuItem key={item.title}>
                                    <SidebarMenuButton asChild>
                                        <Link href={item.url}>
                                            <item.icon />
                                            <span>{item.title}</span>
                                        </Link>
                                    </SidebarMenuButton>
                                </SidebarMenuItem>
                            ))}
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>
        </Sidebar>
    );
}
