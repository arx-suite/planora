import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
    title: "Planora Docs",
    description: "Planora Docs - Documentation for Planora",
};

export default async function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en">
            <body>{children}</body>
        </html>
    );
}
