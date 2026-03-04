"use client";

import { useProfile } from "@/context/profile-context";
import { HeroSection } from "./hero";
import { Footer, Navbar } from "./layout";
import { UserProfileSection } from "./user";

export default function Home() {
    const { user } = useProfile();

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
