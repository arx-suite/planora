import type { ReactNode } from "react";
import { SidebarProvider, SidebarTrigger, WorkspaceNavbar } from "@/components/workspace/layout";
import { WorkspaceProvider } from "@/context/workspace-provider";

const productDevelopmentProjects: Project[] = [
    {
        projectId: "9324-5235-4156-2101",
        name: "api",
        description: "Core API services for internal and external clients",
        role: "admin",
        labels: ["backend", "api", "core"],
    },
    {
        projectId: "9324-5235-4156-2102",
        name: "web-app",
        description: "Primary customer-facing web application",
        role: "member",
        labels: ["frontend", "web"],
    },
    {
        projectId: "9324-5235-4156-2103",
        name: "mobile",
        description: "iOS and Android mobile applications",
        role: "viewer",
        labels: ["mobile", "ios", "android"],
    },
];

const operationGrowthProjects: Project[] = [
    {
        projectId: "8324-5235-4156-3101",
        name: "marketing",
        description: "Marketing campaigns and growth experiments",
        role: "member",
        labels: ["marketing", "growth"],
    },
    {
        projectId: "8324-5235-4156-3102",
        name: "infra",
        description: "Infrastructure, CI/CD pipelines, and cloud resources",
        role: "admin",
        labels: ["infra", "devops", "internal"],
    },
];

// biome-ignore lint: lint/correctness/noUnusedVariables
const projectWorkspace: Workspace = {
    info: {
        orgId: "1123-4242-5252-1413",
        name: "Acme Inc",
        subdomain: "acme",
        plan: "enterprise",
        spaceEnabled: false,
    },
    features: ["audit_logs"],
    projects: [...productDevelopmentProjects, ...operationGrowthProjects],
};

const spaces: Space[] = [
    {
        name: "Product Development",
        description: "Customer-facing products and core application development",
        role: "viewer",
        projects: productDevelopmentProjects,
    },
    {
        name: "Operations and Growth",
        description: "Infrastructure, marketing, and internal operations",
        role: "admin",
        projects: operationGrowthProjects,
    },
];

// biome-ignore lint: lint/correctness/noUnusedVariables
const spaceWorkspace: Workspace = {
    info: {
        orgId: "1123-4242-5252-1413",
        name: "Acme Inc",
        subdomain: "acme",
        plan: "enterprise",
        spaceEnabled: true,
    },
    features: ["spaces", "audit_logs"],
    spaces,
};

export default function WorkspaceRootLayout({
    children,
}: Readonly<{
    children: ReactNode;
}>) {
    return (
        <SidebarProvider>
            <WorkspaceProvider workspace={projectWorkspace}>
                <WorkspaceNavbar />
                <main className="w-full">
                    <SidebarTrigger />
                    <div className="p-4 md:p-6">{children}</div>
                </main>
            </WorkspaceProvider>
        </SidebarProvider>
    );
}
