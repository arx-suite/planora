/* copied from: https://github.com/Cusicon/word-casing */

export function toNameCase(text: string): string {
    if (!text) return "";

    return text
        .toLowerCase()
        .split(" ")
        .filter((word) => word.length > 0)
        .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
        .join(" ");
}

export function toSentenceCase(text: string): string {
    if (!text) return "";

    const sentences = text
        .split(".")
        .filter((sentence) => sentence.trim().length > 0)
        .map((sentence) => {
            const trimmed = sentence.trim();
            if (!trimmed) return "";
            return trimmed.charAt(0).toUpperCase() + trimmed.slice(1);
        });

    let result = sentences.join(". ");

    // Preserve the trailing period if the original had one
    if (text.trim().endsWith(".") && !result.endsWith(".")) {
        result += ".";
    }

    return result;
}
