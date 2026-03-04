import { components } from "@planora/api-types";

declare global {
    export type UserProfile = components["schemas"]["UserProfile"];

    export interface Org {
        organizationId: string;
        ownerId: string;
        name: string;
        subdomain: string;
        plan: string;
    }
}
