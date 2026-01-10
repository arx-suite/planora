"use server";

import { context, trace } from "@opentelemetry/api";
import { z } from "zod";
import { config } from "@/lib/config";
import { attachCookie } from "@/lib/cookie";
import type { SignInFormActionResponse, SignInFormData } from "@/types/auth";

const signinSchema = z.object({
    email: z.email("Please enter a valid email address"),
    password: z
        .string("Please enter a valid password")
        .min(6, "Password must be at least 6 characters"),
});

const tracer = trace.getTracer("auth.signin");

export async function signinAction(
    _prevState: SignInFormActionResponse | null,
    formData: FormData,
): Promise<SignInFormActionResponse> {
    return await context.with(
        trace.setSpan(context.active(), tracer.startSpan("signinAction")),
        async () => {
            const span = trace.getSpan(context.active());
            span?.setAttribute("component", "server-action");
            span?.setAttribute("action.name", "signinAction");

            try {
                const rawData: SignInFormData = {
                    email: formData.get("email") as string,
                    password: formData.get("password") as string,
                };

                span?.addEvent("validating_form_data");
                const parsed = signinSchema.safeParse(rawData);
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

                span?.addEvent("sending_auth_request");

                const response = await fetch(`${config.api}/v1/auth/signin`, {
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

                span?.setStatus({ code: 1, message: "Sign-in success" });
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
                    message: "An unexpected error occurred. Please try again later.",
                };
            }
        },
    );
}
