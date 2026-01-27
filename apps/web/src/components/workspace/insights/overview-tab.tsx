import { Badge, Card, CardContent } from "@planora/ui";
import { AlertTriangle, CheckCircle, Clock, ListTodo } from "lucide-react";

export function OverviewTab() {
    return (
        <div className="space-y-6">
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <StatCard title="Total Tasks" value="1,248" icon={<ListTodo />} />
                <StatCard title="Completed" value="932" icon={<CheckCircle />} />
                <StatCard title="In Progress" value="214" icon={<Clock />} />
                <StatCard title="Blocked" value="32" icon={<AlertTriangle />} danger />
            </div>

            <Card>
                <CardContent className="p-4 flex flex-wrap gap-2">
                    <Badge variant="secondary">18 overdue tasks</Badge>
                    <Badge variant="secondary">7 blocked &gt; 3 days</Badge>
                    <Badge variant="secondary">12 stale tasks</Badge>
                </CardContent>
            </Card>
        </div>
    );
}

function StatCard({
    title,
    value,
    icon,
    danger,
}: {
    title: string;
    value: string;
    icon: React.ReactNode;
    danger?: boolean;
}) {
    return (
        <Card>
            <CardContent className="p-4 space-y-1">
                <div className="flex items-center justify-between text-xs text-muted-foreground">
                    <span>{title}</span>
                    {icon}
                </div>
                <div className={`text-xl font-semibold ${danger ? "text-red-500" : ""}`}>
                    {value}
                </div>
            </CardContent>
        </Card>
    );
}
