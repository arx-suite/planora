declare global {
    export interface Workspace {
        info: Organization;
        features: string[];
        projects?: Project[];
        spaces?: Space[];
    }

    export interface Organization {
        orgId: string;
        name: string;
        subdomain: string;
        spaceEnabled: boolean;
        plan: string;
    }

    export interface Space {
        name: string;
        description: string;
        role: string;
        projects: Project[];
    }

    export interface Project {
        projectId: string;
        name: string;
        description: string;
        role: string;
        labels: string[];
    }
}

export {};
