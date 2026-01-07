"use client";

import { Github, Mail } from "lucide-react";
import { motion } from "motion/react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useActionState, useEffect } from "react";
import { toast } from "sonner";
import { signinAction } from "@/actions/signin";
import { slideLeft } from "@/components/core/motions";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Spinner } from "@/components/ui/spinner";
import type { SignInFormActionResponse } from "@/types/auth";

const initialState: SignInFormActionResponse = {
    success: false,
    message: "",
};

export default function SigninPage() {
    const [state, action, isPending] = useActionState(signinAction, initialState);
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
            <Card className="shadow-xl">
                <CardHeader>
                    <h2 className="text-2xl font-bold text-center">Welcome back</h2>
                    <p className="text-sm text-center text-muted-foreground mt-1">
                        Sign in to continue your collaboration
                    </p>
                </CardHeader>

                <CardContent className="space-y-4">
                    <form action={action} className="flex flex-col gap-5">
                        <div className="flex flex-col gap-1">
                            <Label htmlFor="email" className="text-sm font-medium">
                                Email
                            </Label>
                            <Input
                                name="email"
                                id="email"
                                type="text"
                                defaultValue={state?.values?.email}
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
                                required
                            />
                            {state.errors?.password ? (
                                <p className="text-red-400 font-semibold text-sm">
                                    {state.errors.password}
                                </p>
                            ) : (
                                ""
                            )}
                        </div>
                        <Button className="w-full bg-linear-to-r from-indigo-500 to-violet-500 text-white font-semibold">
                            Sign In{" "}
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
                    <p>Don't have an account? </p>
                    <Link href="/signup" className="text-indigo-500 hover:underline">
                        Create one
                    </Link>
                </CardFooter>
            </Card>
        </motion.div>
    );
}
