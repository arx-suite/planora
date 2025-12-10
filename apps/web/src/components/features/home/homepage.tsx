"use client";

import { useEffect } from "react";
import { Footer } from "@/components/layout/footer";
import { Navbar } from "@/components/layout/navbar";
import { type User, useUser } from "@/context/user-context";
import { HeroSection } from "./hero";
import { UserProfileSection } from "./user";

export default function Home({ userIn }: { userIn: User | null }) {
    const { user, setUser } = useUser();

    useEffect(() => {
        setUser(userIn);
    }, [userIn, setUser]);

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
