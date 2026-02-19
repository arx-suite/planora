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

const signupSchema = z
    .object({
        username: z.string("Please enter a valid username"),
        email: z.email("Please enter a valid email address"),
        password: z
            .string("Please enter a valid password")
            .min(6, "Password must be at least 6 characters"),
        confirmPassword: z.string().min(6, "Confirm password must be at least 6 characters"),
    })
    .refine((data) => data.password === data.confirmPassword, {
        message: "Password does not match",
        path: ["confirmPassword"],
    });

export default function SignupPage() {
    const form = useForm<z.infer<typeof signupSchema>>({
        resolver: zodResolver(signupSchema),
        defaultValues: {
            username: "",
            email: "",
            password: "",
            confirmPassword: "",
        },
    });

    function onSubmit(data: z.infer<typeof signupSchema>) {
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
                    <h2 className="text-2xl font-bold text-center">Create your account</h2>
                    <p className="text-sm text-center text-muted-foreground mt-1">
                        Get started with your team workspace
                    </p>
                </CardHeader>
                <CardContent>
                    <form id="form-signup" onSubmit={form.handleSubmit(onSubmit)}>
                        <FieldGroup>
                            <Controller
                                name="username"
                                control={form.control}
                                render={({ field, fieldState }) => (
                                    <Field data-invalid={fieldState.invalid}>
                                        <FieldLabel htmlFor="form-signup-username">
                                            Username
                                        </FieldLabel>
                                        <Input
                                            {...field}
                                            id="form-signup-username"
                                            type="text"
                                            aria-invalid={fieldState.invalid}
                                            placeholder="yourname"
                                        />
                                        {fieldState.invalid && (
                                            <FieldError errors={[fieldState.error]} />
                                        )}
                                    </Field>
                                )}
                            />
                            <Controller
                                name="email"
                                control={form.control}
                                render={({ field, fieldState }) => (
                                    <Field data-invalid={fieldState.invalid}>
                                        <FieldLabel htmlFor="form-signup-email">Email</FieldLabel>
                                        <Input
                                            {...field}
                                            id="form-signup-email"
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
                                        <FieldLabel htmlFor="form-signup-password">
                                            Password
                                        </FieldLabel>
                                        <Input
                                            {...field}
                                            id="form-signup-password"
                                            type="password"
                                            aria-invalid={fieldState.invalid}
                                            autoComplete="off"
                                            placeholder="••••••"
                                            required
                                        />
                                        {fieldState.invalid && (
                                            <FieldError errors={[fieldState.error]} />
                                        )}
                                    </Field>
                                )}
                            />
                            <Controller
                                name="confirmPassword"
                                control={form.control}
                                render={({ field, fieldState }) => (
                                    <Field data-invalid={fieldState.invalid}>
                                        <FieldLabel htmlFor="form-signup-confirm-password">
                                            Confirm Password
                                        </FieldLabel>
                                        <Input
                                            {...field}
                                            id="form-signup-confirm-password"
                                            type="password"
                                            aria-invalid={fieldState.invalid}
                                            autoComplete="off"
                                            placeholder="••••••"
                                            required
                                        />
                                        {fieldState.invalid && (
                                            <FieldError errors={[fieldState.error]} />
                                        )}
                                    </Field>
                                )}
                            />
                            <Field orientation="horizontal">
                                <Button type="submit" form="form-signup" className="w-full">
                                    Sign up
                                </Button>
                            </Field>
                        </FieldGroup>
                    </form>
                </CardContent>
                <div className="relative">
                    <div className="absolute inset-0 flex items-center">
                        <span className="w-full border-t" />
                    </div>
                    <div className="relative flex justify-center text-xs uppercase">
                        <span className="bg-background px-3 text-muted-foreground">
                            Or continue with
                        </span>
                    </div>
                </div>
                <CardContent className="flex gap-3">
                    <Button variant="outline" className="flex-1 gap-2">
                        <Github className="w-4 h-4" />
                        GitHub
                    </Button>
                    <Button variant="outline" className="flex-1 gap-2">
                        <Mail className="w-4 h-4" />
                        Google
                    </Button>
                </CardContent>
                <CardFooter className="text-center text-sm text-muted-foreground">
                    <p>Already have an account? </p>
                    <Button type="button" variant="link" className="ml-2">
                        <Link href="/signin">Sign in</Link>
                    </Button>
                </CardFooter>
            </Card>
        </motion.div>
    );
}
