"use client";

import { redirect } from "next/navigation";
import { Footer, Navbar } from "@/components/features/home/layout";
import { ProfileSection } from "@/components/features/home/profile";
import { useProfile } from "@/context/profile-context";

export default function ProfilePage() {
    const { user, status } = useProfile();
    if (status === "anonymous" || user === null) redirect("/");

    return (
        <>
            <Navbar />
            <ProfileSection />
            <Footer />
        </>
    );
}
