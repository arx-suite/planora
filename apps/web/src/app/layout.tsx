import type { Metadata } from "next";
import "./globals.css";
import { AppProvider } from "@/context/app-provider";
import { ThemeProvider } from "@/context/theme-provider";
import { ToastProvider } from "@/context/toast-provider";
import { fetchUser } from "@/lib/api/auth";
import { inter, mono, montserrat } from "./fonts";

export const metadata: Metadata = {
    title: "Planora Hall",
    description: "Planora Hall - Collaboration Tool",
};

export default async function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    const user = await fetchUser();

    let profile: Profile | null;

    if (user === null) {
        profile = null;
    } else {
        profile = {
            user,
            ownedOrgs: [],
            joinedOrgs: [],
        };
    }

    return (
        <html
            lang="en"
            className={`${inter.variable} ${montserrat.variable} ${mono.variable} font-sans`}
            suppressHydrationWarning
        >
            <body>
                <ThemeProvider attribute="class" defaultTheme="light" enableSystem>
                    <AppProvider profile={profile}>{children}</AppProvider>
                    <ToastProvider />
                </ThemeProvider>
            </body>
        </html>
    );
}
