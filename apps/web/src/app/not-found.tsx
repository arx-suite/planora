"use client";

import { motion } from "motion/react";
import Link from "next/link";
import { fadeIn, slideUp } from "@/components/core/motions";
import { Button } from "@/components/ui/button";

export default function NotFound() {
    return (
        <main className="flex flex-col items-center justify-center min-h-screen text-center px-6 bg-background">
            <motion.h1
                variants={slideUp}
                initial="hidden"
                animate="show"
                className="text-8xl font-extrabold text-foreground mb-4"
            >
                404
            </motion.h1>

            <motion.h2
                variants={fadeIn}
                initial="hidden"
                animate="show"
                className="text-2xl md:text-3xl font-semibold mb-4"
            >
                Lost in Space 🌌
            </motion.h2>

            <motion.p
                variants={fadeIn}
                initial="hidden"
                animate="show"
                className="text-muted-foreground max-w-md mb-8"
            >
                Looks like the page you're looking for drifted away. Let's get you back to home
                base.
            </motion.p>

            <motion.div variants={fadeIn} initial="hidden" animate="show">
                <Button asChild size="lg">
                    <Link href="/">Return Home</Link>
                </Button>
            </motion.div>

            <motion.div
                variants={fadeIn}
                initial="hidden"
                animate="show"
                className="mt-16 text-sm text-muted-foreground"
            >
                <p>
                    Need help? Check out our{" "}
                    <Link
                        href="https://github.com/planora-proj/planora"
                        className="underline hover:text-foreground"
                    >
                        GitHub project
                    </Link>
                    .
                </p>
            </motion.div>
        </main>
    );
}
