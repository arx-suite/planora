"use client";

import { notFound } from "next/navigation";
import { useWorkspace } from "@/context/workspace-provider";

export default function ProjectsPage() {
    const { info } = useWorkspace();

    if (info.spaceEnabled) notFound();

    return <h1>projects page</h1>;
}
