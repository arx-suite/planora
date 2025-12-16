import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export function BillingTab() {
    return (
        <Card className="rounded-2xl border bg-white/4 backdrop-blur">
            <CardHeader>
                <CardTitle>Billing</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
                <div className="text-sm text-muted-foreground">
                    Billing portal coming soon.
                </div>
            </CardContent>
        </Card>
    );
}
