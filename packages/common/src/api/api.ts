import type { ApiResponse, ApiResult } from "./types";

const isServer = typeof window === "undefined";

export class ApiClient {
    private static async request<T>(url: string, init: RequestInit): Promise<ApiResult<T>> {
        const controller = new AbortController();
        const timeout = setTimeout(() => controller.abort(), 15_000);

        try {
            const res = await fetch(url, {
                ...init,
                signal: controller.signal,
                headers: {
                    "Content-Type": "application/json",
                    ...(init.headers ?? {}),
                },
                cache: isServer ? "no-store" : "default",
            });

            let body: ApiResponse<T>;

            try {
                body = (await res.json()) as ApiResponse<T>;
            } catch {
                body = {
                    success: false,
                    message: "Invalid JSON response",
                };
            }

            return {
                response: body,
                status: res.status,
            };
        } catch (err) {
            return {
                status: 0,
                response: {
                    success: false,
                    message:
                        err instanceof DOMException && err.name === "AbortError"
                            ? "Request timeout"
                            : "Network error",
                },
            };
        } finally {
            clearTimeout(timeout);
        }
    }

    static get<T>(url: string) {
        return ApiClient.request<T>(url, { method: "GET" });
    }

    static post<T, B = unknown>(url: string, body: B) {
        return ApiClient.request<T>(url, {
            method: "POST",
            body: JSON.stringify(body),
        });
    }

    static patch<T, B = unknown>(url: string, body: B) {
        return ApiClient.request<T>(url, {
            method: "PATCH",
            body: JSON.stringify(body),
        });
    }

    static delete<T>(url: string) {
        return ApiClient.request<T>(url, { method: "DELETE" });
    }
}
