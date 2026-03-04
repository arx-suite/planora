"use client";

import { useEffect } from "react";
import Home from "@/components/features/home/homepage";
import { useProfile } from "@/context/profile-context";
import { api } from "@/lib/api/client";

export default function HomePage() {
    const { user, setProfile } = useProfile();

    useEffect(() => {
        if (user !== null) return;

        let profile: UserProfile | null;

        const fetchProfile = async () => {
            try {
                const result = await api.GET("/profile");
                profile = result.data?.payload ?? null;
            } catch (_) {
                profile = null;
            }

            setProfile(profile);
        };

        fetchProfile();
    }, [user]);

    return <Home />;
}
