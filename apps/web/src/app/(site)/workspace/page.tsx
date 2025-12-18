"use client";

import { redirect } from "next/navigation";
import { Footer, Navbar } from "@/components/features/home/layout";
import { OrgSection } from "@/components/features/home/org";
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
