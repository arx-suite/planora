"use client";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@planora/ui";
import { ActivityTab } from "./activity-tab";
import { OverviewTab } from "./overview-tab";
import { UsageTab } from "./usage-tab";

export function InsightSection() {
    return (
        <section className="space-y-4">
            <header>
                <h2 className="text-xl md:text-3xl font-semibold">Workspace Insights</h2>
                <p className="text-sm text-muted-foreground">
                    Insights into activity, usage, and operational health
                </p>
            </header>

            <Tabs defaultValue="overview" className="w-full">
                <TabsList className="w-full justify-start mb-4">
                    <TabsTrigger value="overview">Overview</TabsTrigger>
                    <TabsTrigger value="usage">Usage</TabsTrigger>
                    <TabsTrigger value="activity">Activity</TabsTrigger>
                </TabsList>

                <TabsContent value="overview">
                    <OverviewTab />
                </TabsContent>

                <TabsContent value="usage">
                    <UsageTab />
                </TabsContent>

                <TabsContent value="activity">
                    <ActivityTab />
                </TabsContent>
            </Tabs>
        </section>
    );
}
