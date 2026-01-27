import { Card, CardContent, CardHeader, CardTitle } from "@planora/ui";

export function ActivityTab() {
    return (
        <div className="space-y-4">
            <Card>
                <CardHeader>
                    <CardTitle className="text-sm">Task Activity (Last 30 days)</CardTitle>
                </CardHeader>

                <CardContent className="text-sm text-muted-foreground">
                    Charts and timelines will live here.
                    <br />
                    (Created vs completed, velocity, trends)
                </CardContent>
            </Card>
        </div>
    );
}
