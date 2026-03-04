import { components } from "@planora/api-types";

declare global {
    export type UserProfile = components["schemas"]["UserProfile"];

    // TODO: replace these types
    export interface Org {
        organizationId: string;
        ownerId: string;
        name: string;
        subdomain: string;
        plan: string;
    }

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

    export type ApiResult<T> = {
        success: boolean;
        message: string;
        payload?: T;
    };
}
