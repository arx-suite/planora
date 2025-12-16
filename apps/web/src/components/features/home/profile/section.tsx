import { useState } from "react";
import { ProfileBody, ProfileSidebar, type Tabs } from ".";

export function ProfileSection() {
    const [activeTab, setActiveTab] = useState<Tabs>("profile");

    return (
        <main className="mt-20 min-h-screen max-w-7xl mx-auto px-4 py-8">
            <div className="grid grid-cols-1 md:grid-cols-[260px_1fr] gap-8">
                <ProfileSidebar
                    activeTab={activeTab}
                    setActiveTab={setActiveTab}
                />
                <ProfileBody
                    activeTab={activeTab}
                    setActiveTab={setActiveTab}
                />
            </div>
        </main>
    );
}
