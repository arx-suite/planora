"use client";

import { notFound } from "next/navigation";
import { useWorkspace } from "@/context/workspace-provider";

export default function SpacesPage() {
    const { info } = useWorkspace();

    if (!info.spaceEnabled) notFound();

    return <h1>Spaces Page</h1>;
}
