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

        const fetchData = async () => {
            try {
                const result = await api.GET("/profile");
                profile = result.data?.payload ?? null;
            } catch (error: any) {
                profile = null;
            }

            setProfile(profile);
        };

        fetchData();
    }, []);

    return <Home />;
}
