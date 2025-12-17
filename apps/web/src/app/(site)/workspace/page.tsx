"use client";

import { redirect } from "next/navigation";
import { OrgSection } from "@/components/features/home/org";
import { Footer } from "@/components/layout/footer";
import { Navbar } from "@/components/layout/navbar";
import { useProfile } from "@/context/profile-context";

export default function WorkspacePage() {
    const { user, status } = useProfile();
    if (status === "anonymous" || user === null) redirect("/");

    return (
        <>
            <Navbar />
            <OrgSection />
            <Footer />
        </>
    );
}
