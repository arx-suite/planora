"use client";

import {
    Badge,
    Button,
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    Label,
    Switch,
} from "@planora/ui";
import { cn } from "@planora/ui/lib";
import Link from "next/link";
import { useState } from "react";
import { useWorkspace } from "@/context/workspace-provider";

type Plan = "free" | "pro" | "enterprise";

const PLAN_ORDER: Plan[] = ["free", "pro", "enterprise"];

const hasRequiredPlan = (current: Plan, required: Plan) =>
    PLAN_ORDER.indexOf(current) >= PLAN_ORDER.indexOf(required);

const FEATURE_DEFINITIONS = [
    {
        key: "spaces",
        title: "Spaces",
        description: "Organize projects into logical groups with scoped access and ownership.",
        category: "Organization",
        minPlan: "free" as Plan,
        docsUrl: "/features/spaces",
    },
    {
        key: "audit_logs",
        title: "Audit Logs",
        description:
            "View and export a detailed history of sensitive actions across your organization.",
        category: "Security",
        minPlan: "pro" as Plan,
        docsUrl: "/features/audit-logs",
    },
    {
        key: "custom_roles",
        title: "Custom Roles",
        description: "Define fine-grained permissions for advanced access control scenarios.",
        category: "Access Control",
        minPlan: "enterprise" as Plan,
        docsUrl: "/features/custom-roles",
    },
];

export function FeatureSettings() {
    const { features, info } = useWorkspace();
    const currentPlan = info.plan as Plan;

    const [enabledFeatures, setEnabledFeatures] = useState<string[]>(features);

    const isEnabled = (key: string) => enabledFeatures.includes(key);

    const toggleFeature = (key: string, enabled: boolean) => {
        setEnabledFeatures((prev) => (enabled ? [...prev, key] : prev.filter((f) => f !== key)));
    };

    return (
        <Card>
            <CardHeader>
                <CardTitle>Features</CardTitle>
                <CardDescription>
                    Discover and manage features available for your organization.
                </CardDescription>
            </CardHeader>

            <CardContent className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {FEATURE_DEFINITIONS.map((feature) => {
                    const enabled = isEnabled(feature.key);
                    const allowed = hasRequiredPlan(currentPlan, feature.minPlan);

                    return (
                        <Card
                            key={feature.key}
                            className={cn(
                                "transition-colors",
                                enabled && allowed && "border-primary/40",
                                !allowed && "opacity-70",
                            )}
                        >
                            <Label
                                htmlFor={feature.key}
                                className={cn(
                                    "flex items-start justify-between gap-4 p-4",
                                    allowed ? "cursor-pointer" : "cursor-not-allowed",
                                )}
                            >
                                <div className="space-y-1">
                                    <CardTitle className="text-base">{feature.title}</CardTitle>

                                    <Badge variant="secondary">{feature.category}</Badge>
                                </div>

                                <Switch
                                    id={feature.key}
                                    checked={enabled}
                                    disabled={!allowed}
                                    onCheckedChange={(checked) =>
                                        toggleFeature(feature.key, checked)
                                    }
                                />
                            </Label>

                            <CardContent className="pt-0 px-4 pb-4 space-y-3">
                                <p className="text-sm text-muted-foreground leading-relaxed">
                                    {feature.description}
                                </p>

                                {!allowed && (
                                    <div className="rounded-md border bg-muted/40 p-3 space-y-2">
                                        <p className="text-sm">
                                            Requires the{" "}
                                            <span className="font-medium capitalize">
                                                {feature.minPlan}
                                            </span>{" "}
                                            plan to enable this feature.
                                        </p>

                                        <div className="flex gap-2">
                                            <Button asChild size="sm">
                                                <Link href="/settings/billing">Upgrade plan</Link>
                                            </Button>

                                            <Button asChild size="sm" variant="ghost">
                                                <Link href={feature.docsUrl}>Learn more</Link>
                                            </Button>
                                        </div>
                                    </div>
                                )}
                            </CardContent>
                        </Card>
                    );
                })}
            </CardContent>
        </Card>
    );
}
