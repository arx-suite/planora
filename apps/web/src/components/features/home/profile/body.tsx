"use client";

import { motion } from "motion/react";
import type { ReactNode } from "react";
import type { ProfileSidebarTabProps } from "./shared";
import { BillingTab, ProfileTab, SecurityTab } from "./tabs";

export function ProfileBody({
    activeTab,
    setActiveTab,
}: ProfileSidebarTabProps) {
    return (
        <main>
            <motion.div
                initial={{ opacity: 0, y: 6 }}
                animate={{ opacity: 1, y: 0 }}
                className="space-y-6"
            >
                <div className="flex md:flex-row flex-col md:items-center md:justify-between gap-3 md:gap-1">
                    <div>
                        <h1 className="text-2xl font-semibold">Settings</h1>
                        <p className="text-sm text-muted-foreground">
                            Manage your profile, security, and billing.
                        </p>
                    </div>

                    <div className="flex gap-2 bg-white/3 border border-white/6 rounded-full">
                        <TabButton
                            active={activeTab === "profile"}
                            onClick={() => setActiveTab("profile")}
                        >
                            Profile
                        </TabButton>
                        <TabButton
                            active={activeTab === "security"}
                            onClick={() => setActiveTab("security")}
                        >
                            Security
                        </TabButton>
                        <TabButton
                            active={activeTab === "billing"}
                            onClick={() => setActiveTab("billing")}
                        >
                            Billing
                        </TabButton>
                    </div>
                </div>

                {activeTab === "profile" && <ProfileTab />}
                {activeTab === "security" && <SecurityTab />}
                {activeTab === "billing" && <BillingTab />}
            </motion.div>
        </main>
    );
}

function TabButton({
    children,
    active,
    onClick,
    compact,
}: {
    children: ReactNode;
    active?: boolean;
    onClick?: () => void;
    compact?: boolean;
}) {
    return (
        <button
            onClick={onClick}
            className={`px-3 py-1 rounded-full text-sm transition ${active ? "bg-white text-slate-900 font-medium" : "text-muted-foreground hover:bg-white/2"} ${compact ? "px-2 py-1 text-xs" : ""}`}
        >
            {children}
        </button>
    );
}
