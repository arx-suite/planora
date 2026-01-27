import { Card, CardContent, CardHeader, CardTitle, Progress } from "@planora/ui";

export function UsageTab() {
    return (
        <div className="space-y-4">
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
        </div>
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
