export type Tabs = "profile" | "security" | "billing";

export type ProfileSidebarTabProps = {
    activeTab: Tabs;
    setActiveTab: (tab: Tabs) => void;
};
