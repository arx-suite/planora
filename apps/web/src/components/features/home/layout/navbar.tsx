"use client";

import { motion, useMotionValueEvent, useScroll } from "motion/react";
import Image from "next/image";
import Link from "next/link";
import { useState } from "react";

import { ThemeToggle } from "@/components/core/theme-toggle";
import { Button } from "@/components/ui/button";

export function Navbar() {
    const [scrolled, setScrolled] = useState(false);
    const { scrollY } = useScroll();

    useMotionValueEvent(scrollY, "change", (latest) => {
        setScrolled(latest > 10);
    });

    return (
        <motion.nav
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 0.3 }}
            className={`fixed top-0 z-50 w-full backdrop-blur-md border-b transition-all ${
                scrolled
                    ? "bg-white/80 dark:bg-zinc-900/80 border-zinc-200 dark:border-zinc-800 shadow-sm"
                    : "bg-transparent border-transparent"
            }`}
        >
            <div className="max-w-7xl mx-auto flex items-center justify-between px-6 py-4">
                <Link
                    href="/"
                    className="text-xl font-semibold tracking-tight flex items-center gap-1"
                >
                    <Image width={40} height={40} alt="Planora" src="/planora.png" />
                    {/* <span className="bg-linear-to-r from-indigo-500 to-violet-500 bg-clip-text text-transparent"> */}
                    <span className="bg-[linear-gradient(135deg,var(--color-primary)_0%,var(--color-secondary)_40%,var(--color-accent-purple)_100%)] bg-clip-text text-transparent uppercase font-extrabold text-lg">
                        Planora
                    </span>
                </Link>

                <div className="flex items-center gap-3">
                    <Link href="/signup">
                        {/* <Button className="bg-linear-to-r from-indigo-500 to-violet-500 text-white px-4"> */}
                        <Button className="bg-[linear-gradient(135deg,var(--color-primary)_0%,var(--color-secondary)_40%,var(--color-accent-purple)_100%)] text-white px-4">
                            Get Started
                        </Button>
                    </Link>
                    <ThemeToggle />
                </div>
            </div>
        </motion.nav>
    );
}
