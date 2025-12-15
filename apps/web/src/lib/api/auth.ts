import { cookies } from "next/headers";

import { config } from "@/lib/config";

const API_AUTH_PROFILE = `${config.api}/v1/auth/profile`;
const API_REFRESH_TOKEN = `${config.api}/v1/auth/refresh`;

export async function fetchUser(): Promise<User | null> {
    try {
        const cookieHeader = (await cookies())
            .getAll()
            .map((c) => `${c.name}=${c.value}`)
            .join("; ");

        const res = await fetch(API_AUTH_PROFILE, {
            headers: { Cookie: cookieHeader },
            credentials: "include",
            cache: "no-store",
        });

        const data: ApiResult<User> = await res.json();
        if (!data.success) {
            await refreshToken();
            return await fetchUser();
        }
        return data.payload ?? null;
    } catch (_err) {
        return null;
    }
}

async function refreshToken() {
    const res = await fetch(API_REFRESH_TOKEN, {
        credentials: "include",
    });

    const data = await res.json();
    if (!data.success) {
        throw new Error("authentication failed");
    }
}
