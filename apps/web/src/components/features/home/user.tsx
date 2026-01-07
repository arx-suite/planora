import { motion } from "motion/react";
import Link from "next/link";
import { Card } from "@/components/ui/card";
import { useAuthenticatedProfile } from "@/context/profile-context";

export function UserProfileSection() {
    const { user } = useAuthenticatedProfile();

    const welcomeMessage = `Welcome ${user.username}`.toUpperCase();

    return (
        <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5 }}
            className="mt-12 space-y-10"
        >
            <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                transition={{ delay: 0.2 }}
                className="text-center space-y-3"
            >
                <h1 className="text-3xl font-bold tracking-tight">{welcomeMessage}</h1>
                <p className="text-muted-foreground text-sm max-w-md mx-auto">
                    Navigate to your personal settings or workspace management.
                </p>
            </motion.div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <motion.div
                    whileHover={{ scale: 1.01 }}
                    whileTap={{ scale: 0.99 }}
                    transition={{ type: "spring", stiffness: 300 }}
                >
                    <Card className="rounded-3xl p-6 hover:shadow-xl border bg-card transition cursor-pointer">
                        <Link href="/profile" className="flex flex-col gap-3 h-full">
                            <h3 className="text-xl font-semibold">Profile</h3>
                            <p className="text-sm text-muted-foreground">
                                Manage your personal settings, preferences, and account details.
                            </p>
                        </Link>
                    </Card>
                </motion.div>

                <motion.div
                    whileHover={{ scale: 1.01 }}
                    whileTap={{ scale: 0.99 }}
                    transition={{ type: "spring", stiffness: 300 }}
                >
                    <Card className="rounded-3xl p-6 hover:shadow-xl border bg-card transition cursor-pointer">
                        <Link href="/workspace" className="flex flex-col gap-3 h-full">
                            <h3 className="text-xl font-semibold">Workspace</h3>
                            <p className="text-sm text-muted-foreground">
                                View and manage your organizations, roles, and collaboration tools.
                            </p>
                        </Link>
                    </Card>
                </motion.div>
            </div>
        </motion.div>
    );
}
