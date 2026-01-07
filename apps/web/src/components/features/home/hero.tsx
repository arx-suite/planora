"use client";

import { motion } from "motion/react";
import Link from "next/link";
import { fadeIn, scaleIn, slideUp } from "@/components/core/motions";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";

const features = [
    {
        icon: "💬",
        title: "Seamless Communication",
        desc: "Instant messaging, threads, and mentions to keep everyone aligned.",
    },
    {
        icon: "📅",
        title: "Smart Project Planning",
        desc: "Plan timelines, assign tasks, and track progress visually.",
    },
    {
        icon: "⚡",
        title: "Real-Time Collaboration",
        desc: "Edit docs and brainstorm live — see updates instantly.",
    },
];

const testimonials = [
    {
        quote: "We believe collaboration tools should be transparent and open. That's why our platform is 100% open source — built with the community, for the community.",
        author: "Open Collaboration Ethos",
    },
    {
        quote: "Organize projects, track progress, chat with your team, and host meetings — all in one unified space. Simplicity meets productivity.",
        author: "All-in-One Workspace",
    },
    {
        quote: "Join developers, designers, and creators building together. Every feature we ship is shaped by real feedback and real people.",
        author: "Community-Driven Innovation",
    },
];

export function HeroSection() {
    return (
        <>
            <section className="flex flex-col items-center justify-center text-center">
                <motion.h1
                    variants={slideUp}
                    initial="hidden"
                    animate="show"
                    className="text-5xl md:text-6xl font-bold tracking-tight mb-6"
                >
                    Work Better, Together
                </motion.h1>

                <motion.p
                    variants={fadeIn}
                    initial="hidden"
                    animate="show"
                    className="text-muted-foreground text-lg md:text-xl max-w-2xl mb-10"
                >
                    Plan, collaborate, and build with your team — all in one unified workspace. From
                    ideas to execution, everything stays connected.
                </motion.p>

                <motion.div
                    initial={{ opacity: 0, y: 20 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ delay: 0.2 }}
                >
                    <Link href="/signup">
                        <Button size="lg" className="px-8 py-6 text-lg cursor-pointer">
                            Get Started — It's Free
                        </Button>
                    </Link>
                </motion.div>
            </section>

            <section className="grid lg:grid-cols-3 gap-6 max-w-3xl lg:max-w-6xl mx-auto px-6 md:px-12 py-16">
                {features.map((f) => (
                    <motion.div
                        key={f.title}
                        variants={slideUp}
                        initial="hidden"
                        whileInView="show"
                        viewport={{ once: true }}
                    >
                        <Card className="p-6 hover:shadow-lg transition-shadow h-full">
                            <CardContent className="">
                                <div className="text-primary mb-4">{f.icon}</div>
                                <h3 className="text-xl font-semibold mb-2">{f.title}</h3>
                                <p className="text-muted-foreground">{f.desc}</p>
                            </CardContent>
                        </Card>
                    </motion.div>
                ))}
            </section>

            <section className="py-24 bg-muted/30 text-center px-6 md:px-12">
                <motion.h2
                    variants={slideUp}
                    initial="hidden"
                    whileInView="show"
                    className="text-4xl font-bold mb-4"
                >
                    Collaboration in Real-Time
                </motion.h2>
                <p className="text-muted-foreground mb-10 max-w-2xl mx-auto">
                    Chat, assign tasks, manage documents, and brainstorm in one seamless interface.
                    Stay productive without context switching.
                </p>

                <motion.div
                    variants={scaleIn}
                    initial="hidden"
                    whileInView="show"
                    className="mx-auto max-w-5xl flex justify-center items-center"
                >
                    <img
                        src="/brandings/dashboard-preview.png"
                        alt="Dashboard preview"
                        className="rounded-xl shadow-2xl border border-border"
                    />
                </motion.div>
            </section>

            <section className="py-24 max-w-6xl mx-auto px-6 md:px-12">
                <motion.h2
                    variants={slideUp}
                    initial="hidden"
                    whileInView="show"
                    className="text-3xl font-bold text-center mb-12"
                >
                    Built by the Community, for the Community
                </motion.h2>

                <div className="grid md:grid-cols-3 gap-6">
                    {testimonials.map((t) => (
                        <motion.div
                            key={t.author}
                            variants={fadeIn}
                            initial="hidden"
                            whileInView="show"
                            className="p-6 border rounded-xl bg-background/60 backdrop-blur-sm"
                        >
                            <p className="italic mb-4">“{t.quote}”</p>
                            <div className="text-sm text-muted-foreground">{t.author}</div>
                        </motion.div>
                    ))}
                </div>
            </section>

            <section className="py-24 bg-muted/30 border-t">
                <div className="max-w-4xl mx-auto text-center px-6">
                    <motion.h2
                        variants={slideUp}
                        initial="hidden"
                        whileInView="show"
                        className="text-3xl font-bold mb-4"
                    >
                        Ready to Build Together?
                    </motion.h2>
                    <motion.p
                        variants={fadeIn}
                        initial="hidden"
                        whileInView="show"
                        className="text-lg text-muted-foreground mb-8 leading-relaxed"
                    >
                        We're building the next-generation collaboration platform — open,
                        transparent, and shaped by real teams like yours. Join us early and help
                        shape the future of teamwork.
                    </motion.p>

                    <motion.div
                        variants={fadeIn}
                        initial="hidden"
                        whileInView="show"
                        className="flex justify-center gap-4"
                    >
                        <a
                            href="/signup"
                            className="px-6 py-3 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
                        >
                            Get Started
                        </a>
                        <a
                            href="https://github.com/planora-proj/planora"
                            target="_blank"
                            rel="noopener noreferrer"
                            className="px-6 py-3 rounded-lg border hover:bg-accent hover:text-accent-foreground transition-colors"
                        >
                            Contribute on GitHub
                        </a>
                    </motion.div>
                </div>
            </section>
        </>
    );
}
