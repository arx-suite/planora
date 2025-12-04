import { type ReactNode } from "react";

export default function RootLayout({
    children,
}: Readonly<{
    children: ReactNode;
}>) {
    return <>{children}</>;
}
