"use client";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@planora/ui";
import { FeatureSettings } from "./features-panel";
import { GeneralSettings } from "./general";

export function SettingsTabs() {
    return (
        <Tabs defaultValue="general" className="space-y-6">
            <TabsList className="flex flex-wrap">
                <TabsTrigger value="general">General</TabsTrigger>
                <TabsTrigger value="features">Features</TabsTrigger>
            </TabsList>

            <TabsContent value="general">
                <GeneralSettings />
            </TabsContent>

            <TabsContent value="features">
                <FeatureSettings />
            </TabsContent>
        </Tabs>
    );
}
