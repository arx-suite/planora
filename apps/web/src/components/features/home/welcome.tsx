"use client";

import { motion } from "motion/react";
import { scaleIn, slideUp } from "@/components/core/motions";
import { useUser } from "@/context/user-context";
import { config } from "@/lib/config";
import { cn } from "@/lib/utils";

interface WelcomeProps {
    name: string;
}

export function Welcome({ name }: WelcomeProps) {
    const { setUser } = useUser();

    const handleLogout = async () => {
        try {
            await fetch(`${config.api}/v1/auth/signout`, {
                method: "POST",
                credentials: "include",
            });

            setUser(null);
        } catch (_err) {}
    };

    return (
        <section>
            <motion.div
                variants={scaleIn}
                initial="hidden"
                whileInView="show"
                className="relative flex flex-col items-center justify-center py-20 text-center overflow-hidden"
            >
                <motion.div
                    className="absolute inset-0 bg-linear-to-tr from-indigo-500/10 via-purple-400/10 to-pink-400/10 blur-3xl"
                    variants={scaleIn}
                    initial="hidden"
                    whileInView="show"
                />

                <motion.h1
                    variants={slideUp}
                    initial="hidden"
                    whileInView="show"
                    className={cn(
                        "text-5xl font-bold bg-linear-to-r from-indigo-500 via-purple-500 to-pink-500 bg-clip-text text-transparent",
                        "drop-shadow-sm tracking-tight",
                    )}
                >
                    Welcome back, {name}! 👋
                </motion.h1>

                <motion.p
                    className="text-neutral-600 dark:text-neutral-400 mt-3 text-lg max-w-xl"
                    variants={slideUp}
                    initial="hidden"
                    whileInView="show"
                >
                    We're building this project to make your work easier and
                    smoother. It's still in the early stages — your feedback
                    means a lot 💜 Keep supporting us as we grow together!
                </motion.p>

                <motion.div className="mt-10 flex flex-wrap items-center justify-center gap-4 z-10">
                    <motion.button
                        initial="hidden"
                        whileInView="show"
                        onClick={handleLogout}
                        className="px-6 py-2.5 rounded-xl bg-red-500 text-white hover:bg-red-600 transition-all shadow-md"
                    >
                        Logout
                    </motion.button>
                </motion.div>
            </motion.div>
        </section>
    );
}
