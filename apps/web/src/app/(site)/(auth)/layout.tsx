"use client";

import { motion } from "motion/react";

import { slideRight } from "@/components/core/motions";
import { Navbar } from "@/components/features/home/layout";

export default function AuthLayout({ children }: { children: React.ReactNode }) {
    return (
        <>
            <Navbar />
            <div className="min-h-screen grid lg:grid-cols-2">
                <div className="hidden lg:flex flex-col justify-center items-center px-16 bg-brand/10 border-r border-border">
                    <motion.div
                        variants={slideRight}
                        initial="hidden"
                        animate="show"
                        className="max-w-md text-center"
                    >
                        <div className="space-y-6">
                            <h1 className="text-4xl font-heading font-semibold text-foreground">
                                Collaborate. Plan. Deliver.
                            </h1>
                            <p className="text-muted-foreground text-lg leading-relaxed">
                                Planora brings your team, tasks, and communication together in one
                                place — so you can focus on what matters.
                            </p>
                            <div className="mt-8 text-sm text-muted-foreground italic">
                                “Finally, a workspace that just *feels right.*”
                            </div>
                        </div>
                    </motion.div>
                </div>
                <div className="flex flex-col justify-center items-center px-6 sm:px-12 py-12">
                    {children}
                </div>
            </div>
        </>
    );
}
