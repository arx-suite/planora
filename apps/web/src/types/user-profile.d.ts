declare global {
    export interface User {
        userId: string;
        userTag: string;
        username: string;
        email: string;
        timezone?: string;
        avatarUrl?: string;
    }

    export interface Org {
        organizationId: string;
        ownerId: string;
        name: string;
        subdomain: string;
        plan: string;
    }

    export interface Profile {
        user: User;
        ownedOrgs: Org[];
        joinedOrgs: Org[];
    }
}

export {};
