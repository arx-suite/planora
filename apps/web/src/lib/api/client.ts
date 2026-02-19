import type { paths } from "@planora/api-types";
import createClient from "openapi-fetch";
import { config } from "../config";

const API_VERSION = "v1";

export const api = createClient<paths>({
    baseUrl: `${config.api}/${API_VERSION}`,
    credentials: "include",
});
