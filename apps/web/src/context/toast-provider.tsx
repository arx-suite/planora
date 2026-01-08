"use client";

import { Toaster } from "@planora/ui";

export function ToastProvider() {
    return (
        <Toaster
            position="top-center"
            toastOptions={{
                duration: 3000,
                classNames: {
                    toast: "bg-zinc-900 text-zinc-100 font-bold border border-zinc-800 shadow-lg rounded-xl",
                    success: "bg-green-500",
                    error: "bg-red-600",
                    info: "bg-blue-500",
                    warning: "bg-yellow-500 text-black",
                    title: "text-sm font-medium",
                    description: "text-xs text-zinc-400",
                    actionButton:
                        "bg-zinc-700 hover:bg-zinc-600 text-zinc-100 rounded-md px-2 py-1 text-xs",
                },
            }}
            richColors
            closeButton
        />
    );
}
