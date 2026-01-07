"use client";

import { createContext, type ReactNode, useContext, useState } from "react";

interface OrgProfile {
    owned: Org[];
    joined: Org[];
}

type AnonymousProfile = {
    status: "anonymous";
    user: null;
    orgs: null;
};

type AuthenticatedProfile = {
    status: "authenticated";
    user: User;
    orgs: OrgProfile;
};

export type ProfileState = AnonymousProfile | AuthenticatedProfile;

function mapProfile(profile: Profile | null): ProfileState {
    if (!profile)
        return {
            status: "anonymous",
            user: null,
            orgs: null,
        };

    return {
        status: "authenticated",
        user: profile.user,
        orgs: {
            owned: profile.ownedOrgs,
            joined: profile.joinedOrgs,
        },
    };
}

interface ProfileActions {
    setProfile(profile: Profile | null): void;
    clearProfile(): void;
}

export type ProfileContextValue = ProfileState & ProfileActions;

type ProfileProviderProps = {
    profile: Profile | null;
    children: ReactNode;
};

const ProfileContext = createContext<ProfileContextValue | undefined>(undefined);

export function ProfileProvider({ profile, children }: ProfileProviderProps) {
    const [state, setState] = useState<ProfileState>(() => mapProfile(profile));

    const value: ProfileContextValue = {
        ...state,

        setProfile(profile) {
            setState(mapProfile(profile));
        },

        clearProfile() {
            setState({
                status: "anonymous",
                user: null,
                orgs: null,
            });
        },
    };

    return <ProfileContext.Provider value={value}>{children}</ProfileContext.Provider>;
}

export function useProfile(): ProfileContextValue {
    const ctx = useContext(ProfileContext);
    if (!ctx) {
        throw new Error("useProfile must be used within ProfileProvider");
    }
    return ctx;
}

export function useAuthenticatedProfile(): AuthenticatedProfile {
    const profile = useProfile();

    if (profile.status !== "authenticated") {
        throw new Error("This component requires an authenticated user");
    }

    return profile;
}
