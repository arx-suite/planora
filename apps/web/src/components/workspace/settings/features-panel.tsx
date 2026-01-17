"use client";

import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    Label,
    Separator,
    Switch,
} from "@planora/ui";
import { useState } from "react";
import { useWorkspace } from "@/context/workspace-provider";

const FEATURE_DEFINITIONS = [
    {
        key: "spaces",
        title: "Spaces",
        description: "Group projects into spaces for better organization and access control.",
    },
    {
        key: "audit_logs",
        title: "Audit Logs",
        description:
            "Track important actions across your organization for security and compliance.",
    },
    {
        key: "custom_roles",
        title: "Custom Roles",
        description: "Create fine-grained roles with custom permissions.",
    },
];

export function FeatureSettings() {
    const { features } = useWorkspace();

    const [enabledFeatures, setEnabledFeatures] = useState<string[]>(features);

    const isEnabled = (key: string) => enabledFeatures.includes(key);

    const toggleFeature = (key: string, enabled: boolean) => {
        setEnabledFeatures((prev) => (enabled ? [...prev, key] : prev.filter((f) => f !== key)));
    };

    return (
        <Card>
            <CardHeader>
                <CardTitle>Features</CardTitle>
                <CardDescription>Enable or disable organization-wide features.</CardDescription>
            </CardHeader>

            <CardContent className="space-y-4">
                {FEATURE_DEFINITIONS.map((feature, index) => (
                    <div key={feature.key}>
                        <div className="flex items-center justify-between gap-4">
                            <Label htmlFor={feature.key}>
                                <div>
                                    <p className="font-medium">{feature.title}</p>
                                    <p className="text-sm text-muted-foreground">
                                        {feature.description}
                                    </p>
                                </div>
                            </Label>

                            <Switch
                                id={feature.key}
                                checked={isEnabled(feature.key)}
                                onCheckedChange={(checked) => toggleFeature(feature.key, checked)}
                            />
                        </div>

                        {index !== FEATURE_DEFINITIONS.length - 1 && <Separator className="mt-4" />}
                    </div>
                ))}
            </CardContent>
        </Card>
    );
}
