import { loadEnvConfig } from "@next/env";
import type { NextConfig } from "next";
import path from "path";

loadEnvConfig(path.resolve(__dirname, "../../"));

const nextConfig: NextConfig = {
    output: "standalone",
    /* config options here */
};

export default nextConfig;
