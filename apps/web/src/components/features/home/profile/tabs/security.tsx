import { Button, Card, CardContent, CardHeader, CardTitle, Separator } from "@planora/ui";

export function SecurityTab() {
    return (
        <Card className="rounded-2xl border bg-white/4 backdrop-blur">
            <CardHeader>
                <CardTitle>Security</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
                <div className="flex items-center justify-between">
                    <div>
                        <div className="font-medium">Two-factor Authentication</div>
                        <div className="text-xs text-muted-foreground">Strongly recommended</div>
                    </div>
                    <Button variant="ghost">Enable</Button>
                </div>
                <Separator />
                <div className="flex justify-end gap-2">
                    <Button variant="ghost">Rotate API Key</Button>
                    <Button variant="destructive">Revoke Sessions</Button>
                </div>
            </CardContent>
        </Card>
    );
}
