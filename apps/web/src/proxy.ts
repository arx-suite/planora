import { type NextRequest, NextResponse } from "next/server";

export function proxy(request: NextRequest) {
    const { pathname } = request.nextUrl;
    const subdomain = extractSubdomain(request);

    if (subdomain) {
        if (pathname === "/")
            return NextResponse.rewrite(
                new URL(`/workspace/dashboard`, request.url),
            );

        if (pathname === "/dashboard")
            return NextResponse.redirect(new URL("/", request.url));

        return NextResponse.rewrite(
            new URL(`/workspace${pathname}`, request.url),
        );
    }

    return NextResponse.next();
}

function extractSubdomain(req: NextRequest): string | null {
    const { hostname } = req.nextUrl;
    const host = req.headers.get("host") || "";
    const hostnameWithSubdomain = host.split(":")[0];

    if (hostnameWithSubdomain.includes(hostname)) {
        const parts = hostnameWithSubdomain.split(`.${hostname}`);
        return parts.length > 1 ? parts[0] : null;
    }

    return null;
}

export const config = {
    matcher: [
        /*
         * Match all paths except for:
         * 1. /api routes
         * 2. /_next (Next.js internals)
         * 3. all root files inside /public (e.g. /favicon.ico)
         */
        "/((?!api|_next|[\\w-]+\\.\\w+).*)",
    ],
};
