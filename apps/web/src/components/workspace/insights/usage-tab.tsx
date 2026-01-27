import { Card, CardContent, CardHeader, CardTitle, Progress } from "@planora/ui";

export function UsageTab() {
    return (
        <div className="grid gap-6 lg:grid-cols-2">
            <StorageCard title="Database Storage" usedLabel="1.6 GB used" percentage={64} />
            <StorageCard title="File Storage" usedLabel="800 MB used" percentage={40} />

            <Card className="lg:col-span-2">
                <CardHeader>
                    <CardTitle>Resource Counts</CardTitle>
                </CardHeader>
                <CardContent className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
                    <CountItem label="Tasks" value="1,284" />
                    <CountItem label="Completed Tasks" value="842" />
                    <CountItem label="Attachments" value="356" />
                    <CountItem label="Projects" value="18" />
                </CardContent>
            </Card>
        </div>
    );
}

function StorageCard({
    title,
    usedLabel,
    percentage,
}: {
    title: string;
    usedLabel: string;
    percentage: number;
}) {
    return (
        <Card>
            <CardHeader>
                <CardTitle>{title}</CardTitle>
            </CardHeader>
            <CardContent className="space-y-2">
                <div className="flex justify-between text-sm text-muted-foreground">
                    <span>{usedLabel}</span>
                    <span>{percentage}%</span>
                </div>
                <Progress value={percentage} />
            </CardContent>
        </Card>
    );
}

function CountItem({ label, value }: { label: string; value: string }) {
    return (
        <div className="space-y-1">
            <div className="text-sm text-muted-foreground">{label}</div>
            <div className="text-lg font-medium">{value}</div>
        </div>
    );
}
