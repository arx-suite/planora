"use client";

import { ProjectHeader } from "./layout";

const projectData = {
    name: "API Gateway",
    description: "API Gateway",
};

export function ProjectSection() {
    return (
        <main className="flex flex-col gap-6">
            <ProjectHeader name={projectData.name} />
        </main>
    );
}
