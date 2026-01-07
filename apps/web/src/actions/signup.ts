"use server";

import { context, trace } from "@opentelemetry/api";
import { z } from "zod";
import { config } from "@/lib/config";
import { attachCookie } from "@/lib/cookie";
import type { SignUpFormActionResponse, SignUpFormData } from "@/types/auth";

const signupSchema = z.object({
    username: z.string("Please enter a valid username"),
    email: z.email("Please enter a valid email address"),
    password: z
        .string("Please enter a valid password")
        .min(6, "Password must be at least 6 characters"),
});

const tracer = trace.getTracer("auth.signup");

export async function signupAction(
    _prevState: SignUpFormActionResponse | null,
    formData: FormData,
): Promise<SignUpFormActionResponse> {
    return await context.with(
        trace.setSpan(context.active(), tracer.startSpan("signupAction")),
        async () => {
            const span = trace.getSpan(context.active());
            span?.setAttribute("component", "server-action");
            span?.setAttribute("action.name", "signupAction");

            try {
                const password = formData.get("password") as string;
                const confirmPassword = formData.get("confirm") as string;

                const rawData: SignUpFormData = {
                    username: formData.get("username") as string,
                    email: formData.get("email") as string,
                    password: password,
                };

                span?.addEvent("validating_form_data");
                const parsed = signupSchema.safeParse(rawData);
                if (!parsed.success) {
                    span?.setStatus({ code: 2, message: "Validation failed" });
                    span?.addEvent("form_validation_failed", parsed.error.flatten().fieldErrors);
                    span?.end();

                    return {
                        success: false,
                        message: "Please fix the errors in the form",
                        errors: parsed.error.flatten().fieldErrors,
                        values: rawData,
                    };
                }

                span?.addEvent("validating_password_and_confirm_password");
                if (password !== confirmPassword)
                    return {
                        success: false,
                        message: "password does not match",
                        values: rawData,
                    };

                span?.addEvent("sending_auth_request");

                const response = await fetch(`${config.internal_api}/v1/auth/signup`, {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify(parsed.data),
                });

                span?.setAttribute("response.status_code", response.status);

                const data = await response.json();
                if (!data.success) {
                    span?.setStatus({
                        code: 2,
                        message: "Authentication failed",
                    });
                    span?.addEvent("api_response_error", {
                        message: data.message,
                    });
                    span?.end();

                    return {
                        success: false,
                        message: data.message,
                        values: rawData,
                    };
                }

                span?.setStatus({ code: 1, message: "forward session token" });
                span?.end();

                const setCookieHeaders = response.headers.getSetCookie();
                await attachCookie(setCookieHeaders);

                span?.setStatus({ code: 1, message: "Sign-up success" });
                span?.addEvent("redirecting_to_home");
                span?.end();

                return {
                    success: true,
                    message: data.message,
                    redirectTo: "/",
                };
            } catch (error) {
                span?.recordException(error as Error);
                span?.setStatus({ code: 2, message: "Unexpected error" });
                span?.end();

                return {
                    success: false,
                    message: "An unexpected error occurred. Please try again later",
                };
            }
        },
    );
}
