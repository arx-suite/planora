"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import {
    Button,
    Card,
    CardContent,
    CardFooter,
    CardHeader,
    Field,
    FieldError,
    FieldGroup,
    FieldLabel,
    Input,
} from "@planora/ui";
import { motion, slide } from "@planora/ui/animation";
import { Github, Mail } from "@planora/ui/icons";
import Link from "next/link";
import type * as React from "react";
import { Controller, useForm } from "react-hook-form";
import * as z from "zod";

const signinSchema = z.object({
    email: z.email("Please enter a valid email address"),
    password: z
        .string("Please enter a valid password")
        .min(6, "Password must be at least 6 characters"),
});

export default function SigninPage() {
    const form = useForm<z.infer<typeof signinSchema>>({
        resolver: zodResolver(signinSchema),
        defaultValues: {
            email: "",
            password: "",
        },
    });

    function onSubmit(data: z.infer<typeof signinSchema>) {
        form.reset();
    }

    return (
        <motion.div
            variants={slide("left")}
            initial="hidden"
            animate="show"
            className="w-full max-w-lg"
        >
            <Card className="w-full sm:max-w-md">
                <CardHeader>
                    <h2 className="text-2xl font-bold text-center">Welcome back</h2>
                    <p className="text-sm text-center text-muted-foreground mt-1">
                        Sign in to continue your collaboration
                    </p>
                </CardHeader>
                <CardContent>
                    <form id="form-signin" onSubmit={form.handleSubmit(onSubmit)}>
                        <FieldGroup>
                            <Controller
                                name="email"
                                control={form.control}
                                render={({ field, fieldState }) => (
                                    <Field data-invalid={fieldState.invalid}>
                                        <FieldLabel htmlFor="form-signin-email">Email</FieldLabel>
                                        <Input
                                            {...field}
                                            id="form-signin-email"
                                            type="email"
                                            aria-invalid={fieldState.invalid}
                                            placeholder="your@email.com"
                                        />
                                        {fieldState.invalid && (
                                            <FieldError errors={[fieldState.error]} />
                                        )}
                                    </Field>
                                )}
                            />
                            <Controller
                                name="password"
                                control={form.control}
                                render={({ field, fieldState }) => (
                                    <Field data-invalid={fieldState.invalid}>
                                        <FieldLabel htmlFor="form-signin-password">
                                            Password
                                        </FieldLabel>
                                        <Input
                                            {...field}
                                            id="form-signin-password"
                                            type="password"
                                            aria-invalid={fieldState.invalid}
                                            autoComplete="off"
                                            placeholder="••••••••"
                                            required
                                        />
                                        {fieldState.invalid && (
                                            <FieldError errors={[fieldState.error]} />
                                        )}
                                    </Field>
                                )}
                            />
                        </FieldGroup>
                    </form>
                </CardContent>
                <CardFooter>
                    <Field orientation="horizontal">
                        <Button type="submit" form="form-signin" className="w-full">
                            Sign In
                        </Button>
                    </Field>
                </CardFooter>
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
                <CardFooter className="text-center text-sm text-muted-foreground">
                    <p>Don't have an account? </p>
                    <Button type="button" variant="link" className="ml-2">
                        <Link href="/signup">Create one</Link>
                    </Button>
                </CardFooter>
            </Card>
        </motion.div>
    );
}
