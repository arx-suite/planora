"use client";

import { Badge, Card, CardContent, CardHeader, CardTitle, Progress } from "@planora/ui";
import { AlertTriangle, CheckCircle, Clock, ListTodo } from "lucide-react";

export function InsightSection() {
    return (
        <section className="space-y-6">
            <header>
                <h2 className="text-xl font-semibold">Organization Insights</h2>
                <p className="text-sm text-muted-foreground">
                    Activity, usage, and operational health across your organization
                </p>
            </header>

            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <StatCard
                    title="Total Tasks"
                    value="1,248"
                    icon={<ListTodo className="h-4 w-4" />}
                />
                <StatCard
                    title="Completed"
                    value="932"
                    icon={<CheckCircle className="h-4 w-4" />}
                />
                <StatCard title="In Progress" value="214" icon={<Clock className="h-4 w-4" />} />
                <StatCard
                    title="Blocked"
                    value="32"
                    tone="danger"
                    icon={<AlertTriangle className="h-4 w-4" />}
                />
            </div>

            <Card>
                <CardHeader>
                    <CardTitle className="text-sm">Resource Usage</CardTitle>
                </CardHeader>
                <CardContent className="space-y-4">
                    <UsageRow name="Estimated Hours" value={820} total={1000} />
                    <UsageRow name="Actual Hours" value={910} total={1000} />
                    <UsageRow name="Capacity Used" value={91} total={100} suffix="%" />
                </CardContent>
            </Card>

            <Card>
                <CardHeader>
                    <CardTitle className="text-sm">Operational Signals</CardTitle>
                </CardHeader>
                <CardContent className="flex flex-wrap gap-2">
                    <Badge variant="secondary">18 overdue tasks</Badge>
                    <Badge variant="secondary">7 tasks blocked &gt; 3 days</Badge>
                    <Badge variant="secondary">12 stale tasks</Badge>
                </CardContent>
            </Card>
        </section>
    );
}

function StatCard({
    title,
    value,
    icon,
    tone,
}: {
    title: string;
    value: string;
    icon: React.ReactNode;
    tone?: "danger";
}) {
    return (
        <Card>
            <CardContent className="p-4 space-y-1">
                <div className="flex items-center justify-between text-xs text-muted-foreground">
                    <span>{title}</span>
                    {icon}
                </div>
                <div className={`text-xl font-semibold ${tone === "danger" ? "text-red-500" : ""}`}>
                    {value}
                </div>
            </CardContent>
        </Card>
    );
}

function UsageRow({
    name,
    value,
    total,
    suffix,
}: {
    name: string;
    value: number;
    total: number;
    suffix?: string;
}) {
    const percentage = Math.min((value / total) * 100, 100);

    return (
        <div className="space-y-1">
            <div className="flex justify-between text-xs">
                <span>{name}</span>
                <span className="text-muted-foreground">
                    {value}
                    {suffix ?? ""} / {total}
                    {suffix ?? ""}
                </span>
            </div>
            <Progress value={percentage} />
        </div>
    );
}
