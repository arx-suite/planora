import { maintainer, social } from "@planora/common/data";
import { slide } from "@planora/ui/animation";
import { Github } from "lucide-react";
import { motion } from "motion/react";
import Link from "next/link";

export function Footer() {
    return (
        <footer className="border-t bg-background/50 backdrop-blur-sm">
            <div className="max-w-5xl mx-auto px-6 py-12 md:py-16">
                <motion.div
                    variants={slide("left")}
                    initial="hidden"
                    whileInView="show"
                    className="flex flex-col md:flex-row justify-between items-center gap-6"
                >
                    <div className="text-center md:text-left">
                        <Link href="/" className="text-xl font-bold">
                            Arx - Planora
                        </Link>
                        <p className="text-sm text-muted-foreground mt-2">
                            An open-source collaboration tool built for teams who love to create
                            together.
                        </p>
                    </div>

                    <div className="flex items-center space-x-3">
                        <Link
                            href={social.github.org}
                            target="_blank"
                            rel="noopener noreferrer"
                            className="flex items-center space-x-2 text-muted-foreground hover:text-foreground transition-colors"
                        >
                            <Github className="h-5 w-5" />
                            <span className="text-sm font-medium">View on GitHub</span>
                        </Link>
                    </div>
                </motion.div>

                <motion.div
                    variants={slide("left")}
                    initial="hidden"
                    whileInView="show"
                    className="mt-8 pt-6 border-t text-center text-sm text-muted-foreground"
                >
                    <p>© {new Date().getFullYear()} Planora.</p>
                    <Link
                        href={maintainer.github}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="bg-linear-to-r from-blue-400 via-purple-400 to-pink-400 bg-clip-text text-transparent font-semibold hover:blur-[0.3px]"
                    >
                        Maintained by @{maintainer.name}
                    </Link>
                    .
                </motion.div>
            </div>
        </footer>
    );
}
