"use client";

import { SettingsTabs } from "./settings-tab";

export function OrganizationSettingsSection() {
    return (
        <div className="mx-auto max-w-3xl space-y-6">
            <div>
                <h1 className="text-2xl font-semibold tracking-tight">Organization Settings</h1>
                <p className="text-sm text-muted-foreground">
                    Manage your organization preferences, members, and features.
                </p>
            </div>

            <SettingsTabs />
        </div>
    );
}
