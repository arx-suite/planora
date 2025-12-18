"use client";

import { useEffect } from "react";
import { useProfile } from "@/context/profile-context";
import { HeroSection } from "./hero";
import { Footer, Navbar } from "./layout";
import { UserProfileSection } from "./user";

export default function Home({ profile }: { profile: Profile | null }) {
    const { user, setProfile } = useProfile();

    useEffect(() => {
        setProfile(profile);
    }, [profile]);

    return (
        <div className="min-h-screen bg-background text-foreground">
            <Navbar />

            <main className="py-24 px-6 md:px-12">
                {user ? (
                    <section className="mb-16">
                        <UserProfileSection />
                    </section>
                ) : (
                    <HeroSection />
                )}
            </main>

            <Footer />
        </div>
    );
}
