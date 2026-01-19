import { Inter, JetBrains_Mono, Montserrat } from "next/font/google";

export const inter = Inter({
    subsets: ["latin"],
    variable: "--font-sans",
    weight: ["400", "500", "600", "700"],
    display: "swap",
});

export const montserrat = Montserrat({
    subsets: ["latin"],
    variable: "--font-heading",
    weight: ["500", "600", "700"],
    display: "swap",
});

export const mono = JetBrains_Mono({
    subsets: ["latin"],
    variable: "--font-mono",
    weight: ["400", "500", "600"],
    display: "swap",
});
