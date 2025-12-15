import type { ReactNode } from "react";
import { ProfileProvider } from "./profile-context";

type AppProviderProps = {
    profile: Profile | null;
    children: ReactNode;
};

export function AppProvider({ profile, children }: AppProviderProps) {
    return <ProfileProvider profile={profile}>{children}</ProfileProvider>;
}
