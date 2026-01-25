"use client";

import { Button, Card, CardContent, CardHeader, CardTitle, Input, Label } from "@planora/ui";
import { useState } from "react";
import { useWorkspace } from "@/context/workspace-provider";

export function GeneralSettings() {
    const { info } = useWorkspace();
    const [isEditing, setIsEditing] = useState(false);

    return (
        <Card>
            <CardHeader className="flex flex-row items-center justify-between">
                <div>
                    <CardTitle>General Settings</CardTitle>
                    <p className="text-sm text-muted-foreground">
                        Manage your organization profile and defaults.
                    </p>
                </div>

                {!isEditing && (
                    <Button variant="outline" onClick={() => setIsEditing(true)}>
                        Edit
                    </Button>
                )}
            </CardHeader>

            <CardContent className="space-y-8">
                <section className="space-y-4">
                    <h3 className="text-sm font-medium text-muted-foreground">Organization</h3>

                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div className="space-y-2">
                            <Label>Organization name</Label>
                            <Input
                                className="text-sm"
                                defaultValue={info.name}
                                readOnly={!isEditing}
                            />
                        </div>

                        <div className="space-y-2">
                            <Label>Subdomain</Label>
                            <Input defaultValue={`${info.subdomain}.planora.sbs`} disabled />
                            <p className="text-muted-foreground">
                                Subdomains cannot be changed after creation.
                            </p>
                        </div>
                    </div>
                </section>

                {isEditing && (
                    <div className="flex justify-end gap-2">
                        <Button variant="ghost" onClick={() => setIsEditing(false)}>
                            Cancel
                        </Button>
                        <Button>Save changes</Button>
                    </div>
                )}
            </CardContent>
        </Card>
    );
}
