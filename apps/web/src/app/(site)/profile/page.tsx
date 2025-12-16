"use client";

import { redirect } from "next/navigation";
import { ProfileSection } from "@/components/features/home/profile";
import { Footer } from "@/components/layout/footer";
import { Navbar } from "@/components/layout/navbar";
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
