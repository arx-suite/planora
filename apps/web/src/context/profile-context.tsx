"use client";

import { createContext, type ReactNode, useContext, useState } from "react";

type AnonymousProfile = {
    status: "anonymous";
    user: null;
};

type AuthenticatedProfile = {
    status: "authenticated";
    user: UserProfile;
};

export type ProfileState = AnonymousProfile | AuthenticatedProfile;

function mapProfile(user: UserProfile | null): ProfileState {
    if (!user)
        return {
            status: "anonymous",
            user: null,
        };

    return {
        status: "authenticated",
        user: user,
    };
}

interface ProfileActions {
    setProfile(profile: UserProfile | null): void;
    clearProfile(): void;
}

export type ProfileContextValue = ProfileState & ProfileActions;

type ProfileProviderProps = {
    profile: UserProfile | null;
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
