"use client";

import { createContext, type ReactNode, useContext, useState } from "react";

const WorkspaceContext = createContext<Workspace | undefined>(undefined);

type WorkspaceProviderProps = {
    workspace: Workspace;
    children: ReactNode;
};

interface WorkspaceActions {
    setWorkspace(workspace: Workspace | null): void;
}

export function WorkspaceProvider({ workspace, children }: WorkspaceProviderProps) {
    const [state, _setState] = useState<Workspace>(workspace);

    return <WorkspaceContext.Provider value={state}>{children}</WorkspaceContext.Provider>;
}

export function useWorkspace(): Workspace {
    const ctx = useContext(WorkspaceContext);

    if (!ctx) throw new Error("useWorkspace must be used within WorkspaceProvider");

    return ctx;
}
