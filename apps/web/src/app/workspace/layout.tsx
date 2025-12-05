import type { ReactNode } from "react";
import { WorkspaceNavbar } from "@/components/features/workspace/sidebar";
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";

export default function WorkspaceRootLayout({
    children,
}: Readonly<{
    children: ReactNode;
}>) {
    return (
        <SidebarProvider>
            <WorkspaceNavbar />
            <main className="w-full">
                <SidebarTrigger />
                <div className="p-4 md:p-6 ">{children}</div>
            </main>
        </SidebarProvider>
    );
}
