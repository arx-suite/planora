import { CreditCard, LogOut, Settings, ShieldCheck } from "lucide-react";
import type { ReactNode } from "react";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { useAuthenticatedProfile } from "@/context/profile-context";
import type { ProfileSidebarTabProps } from ".";

export function ProfileSidebar({
    activeTab,
    setActiveTab,
}: ProfileSidebarTabProps) {
    const { user } = useAuthenticatedProfile();

    return (
        <aside className="hidden md:block sticky top-6 self-start">
            <div className="w-full rounded-2xl border border-white/6 bg-white/4 backdrop-blur p-4 shadow-md">
                <div className="flex items-center gap-4">
                    <Avatar className="h-14 w-14">
                        <AvatarImage src={user.avatarUrl} />
                        <AvatarFallback>
                            {user.username.slice(0, 2).toUpperCase()}
                        </AvatarFallback>
                    </Avatar>
                    <div>
                        <div className="text-sm font-semibold">
                            {user.username}
                        </div>
                        <div className="text-xs text-muted-foreground">
                            {user.email}
                        </div>
                    </div>
                </div>

                <Separator className="my-4" />

                <nav className="flex flex-col gap-1">
                    <SidebarItem
                        icon={<Settings className="w-4 h-4" />}
                        title="Profile"
                        active={activeTab === "profile"}
                        onClick={() => setActiveTab("profile")}
                    />

                    <SidebarItem
                        icon={<ShieldCheck className="w-4 h-4" />}
                        title="Security"
                        active={activeTab === "security"}
                        onClick={() => setActiveTab("security")}
                    />

                    <SidebarItem
                        icon={<CreditCard className="w-4 h-4" />}
                        title="Billing"
                        active={activeTab === "billing"}
                        onClick={() => setActiveTab("billing")}
                    />

                    <div className="mt-3 pt-3 border-t border-white/6">
                        <Button
                            variant="ghost"
                            className="w-full justify-start"
                        >
                            <LogOut className="w-4 h-4 mr-2" /> Sign out
                        </Button>
                    </div>
                </nav>
            </div>
        </aside>
    );
}

function SidebarItem({
    icon,
    title,
    active,
    onClick,
}: {
    icon: ReactNode;
    title: string;
    active?: boolean;
    onClick?: () => void;
}) {
    return (
        <button
            onClick={onClick}
            className={`w-full flex items-center gap-3 p-2 rounded-md text-sm transition ${active ? "bg-white/6" : "hover:bg-white/3"}`}
        >
            <div className="text-muted-foreground">{icon}</div>
            <div
                className={`flex-1 text-left ${active ? "font-medium" : "text-sm"}`}
            >
                {title}
            </div>
        </button>
    );
}
