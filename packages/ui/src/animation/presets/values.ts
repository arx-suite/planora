// duration tiers
export const DURATION = {
    instant: 0.12,
    fast: 0.18,
    normal: 0.28,
    slow: 0.45,
} as const;

// easing tokens
export const EASING = {
    standard: [0.2, 0.0, 0.0, 1.0],
    enter: [0.0, 0.0, 0.2, 1.0],
    exit: [0.4, 0.0, 1.0, 1.0],
    emphasis: [0.34, 1.56, 0.64, 1],
} as const;

// distant rules
export const DISTANCE = {
    micro: 4,
    small: 8,
    medium: 16,
    large: 32,
} as const;
