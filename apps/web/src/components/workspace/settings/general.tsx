import { Button, Card, CardContent, CardHeader, CardTitle, Input } from "@planora/ui";
import { useWorkspace } from "@/context/workspace-provider";

export function GeneralSettings() {
    const { info } = useWorkspace();

    return (
        <Card>
            <CardHeader>
                <CardTitle>Organization Profile</CardTitle>
            </CardHeader>

            <CardContent className="space-y-4">
                <Input placeholder="Organization name" defaultValue={info.name} />
                <Input placeholder="Subdomain" disabled defaultValue={info.subdomain} />
                <Input placeholder="Default timezone" defaultValue="Asia/Pacific" />
                <Input placeholder="Default language" defaultValue="en_US" />

                <div className="flex justify-end">
                    <Button>Save changes</Button>
                </div>
            </CardContent>
        </Card>
    );
}
