"use client";

import {
    Button,
    Card,
    CardContent,
    CardFooter,
    CardHeader,
    Input,
    Label,
    Spinner,
} from "@planora/ui";
import { Github, Mail } from "lucide-react";
import { motion } from "motion/react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useActionState, useEffect } from "react";
import { toast } from "sonner";
import { signupAction } from "@/actions/signup";
import { slideLeft } from "@/components/core/motions";
import type { SignUpFormActionResponse } from "@/types/auth";

const initialState: SignUpFormActionResponse = {
    success: false,
    message: "",
};

export default function SignupPage() {
    const [state, action, isPending] = useActionState(signupAction, initialState);
    const router = useRouter();

    useEffect(() => {
        if (!state) return;

        if (!state?.success) {
            if (state?.message.trim().length !== 0) {
                toast.error(`Error: ${state.message}`);
            }
            return;
        }
        setTimeout(() => {
            if (state?.message.trim().length !== 0) {
                toast.success(`Success: ${state.message}`);
            }
            router.push(state.redirectTo || "/");
            router.refresh();
        }, 200);
    }, [state, router]);

    return (
        <motion.div
            variants={slideLeft}
            initial="hidden"
            animate="show"
            className="w-full max-w-lg"
        >
            <Card className="w-full max-w-md bg-white/80 dark:bg-zinc-900/80 backdrop-blur-xl border border-zinc-200 dark:border-zinc-800 shadow-xl">
                <CardHeader>
                    <h2 className="text-2xl font-bold text-center">Create your account</h2>
                    <p className="text-sm text-center text-muted-foreground mt-1">
                        Get started with your team workspace
                    </p>
                </CardHeader>

                <CardContent className="space-y-4">
                    <form action={action} className="flex flex-col gap-5">
                        <div className="flex flex-col gap-1">
                            <Label htmlFor="username" className="text-sm font-medium">
                                Username
                            </Label>
                            <Input
                                defaultValue={state?.values?.username}
                                id="username"
                                name="username"
                                type="text"
                                placeholder="yourname"
                            />
                            {state.errors?.username ? (
                                <p className="text-red-400 font-semibold text-sm">
                                    {state.errors.username}
                                </p>
                            ) : (
                                ""
                            )}
                        </div>
                        <div className="flex flex-col gap-1">
                            <Label htmlFor="email" className="text-sm font-medium">
                                Email
                            </Label>
                            <Input
                                defaultValue={state?.values?.email}
                                name="email"
                                id="email"
                                type="email"
                                placeholder="your@email.com"
                            />
                            {state.errors?.email ? (
                                <p className="text-red-400 font-semibold text-sm">
                                    {state.errors.email}
                                </p>
                            ) : (
                                ""
                            )}
                        </div>
                        <div className="flex flex-col gap-1">
                            <Label htmlFor="password" className="text-sm font-medium">
                                Password
                            </Label>
                            <Input
                                name="password"
                                id="password"
                                type="password"
                                placeholder="••••••••"
                            />
                            {state.errors?.password ? (
                                <p className="text-red-400 font-semibold text-sm">
                                    {state.errors.password}
                                </p>
                            ) : (
                                ""
                            )}
                        </div>
                        <div className="flex flex-col gap-1">
                            <Label htmlFor="confirm" className="text-sm font-medium">
                                Confirm Password
                            </Label>
                            <Input
                                name="confirm"
                                id="confirm"
                                type="password"
                                placeholder="••••••••"
                            />
                        </div>

                        <Button className="w-full bg-linear-to-r from-indigo-500 to-violet-500 text-white font-semibold">
                            Create Account
                            {isPending ? (
                                <span className="ml-2">
                                    <Spinner />
                                </span>
                            ) : (
                                ""
                            )}
                        </Button>
                    </form>
                    <div className="relative py-2">
                        <div className="absolute inset-0 flex items-center">
                            <span className="w-full border-t" />
                        </div>
                        <div className="relative flex justify-center text-xs uppercase">
                            <span className="bg-background px-2 text-muted-foreground">
                                or continue with
                            </span>
                        </div>
                    </div>

                    <div className="flex justify-center gap-4">
                        <Button variant="outline" className="flex items-center gap-2">
                            <Github className="w-4 h-4" /> GitHub
                        </Button>
                        <Button variant="outline" className="flex items-center gap-2">
                            <Mail className="w-4 h-4" /> Google
                        </Button>
                    </div>
                </CardContent>

                <CardFooter className="flex gap-2 text-center text-sm text-muted-foreground">
                    Already have an account?{" "}
                    <Link href="/signin" className="text-indigo-500 hover:underline">
                        Sign in
                    </Link>
                </CardFooter>
            </Card>
        </motion.div>
    );
}
