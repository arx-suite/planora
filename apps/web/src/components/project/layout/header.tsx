import { Button, H1 } from "@planora/ui";
import { MoveLeft } from "lucide-react";
import Link from "next/link";

type ProjectHeaderProps = {
    name: string;
};

export function ProjectHeader({ name }: ProjectHeaderProps) {
    return (
        <header>
            <Link href="/projects">
                <Button variant="link">
                    <MoveLeft /> Back To Projects
                </Button>
            </Link>
            <div className="flex items-center justify-between">
                <H1 className="mt-2">{name}</H1>
                <div className="flex items-center gap-2">
                    <Link href="/projects">
                        <Button variant="outline" size="sm">
                            settings
                        </Button>
                    </Link>
                </div>
            </div>
        </header>
    );
}
