import { DISTANCE, DURATION, EASING } from "./values";

export type AnimationPreset = {
    initial: Record<string, any>;
    animate: Record<string, any>;
    exit?: Record<string, any>;
    transition: Record<string, any>;
};

export const slide = (direction: "up" | "down" | "left" | "right"): AnimationPreset => {
    const axis = direction === "left" || direction === "right" ? "x" : "y";
    const value = direction === "up" || direction === "left" ? DISTANCE.medium : -DISTANCE.medium;

    return {
        initial: { opacity: 0, [axis]: value },
        animate: { opacity: 1, [axis]: 0 },
        exit: { opacity: 0, [axis]: value / 2 },
        transition: {
            duration: DURATION.normal,
            ease: EASING.standard,
        },
    };
};

export const pop: AnimationPreset = {
    initial: { opacity: 0, scale: 0.9 },
    animate: { opacity: 1, scale: 1 },
    transition: {
        duration: DURATION.fast,
        ease: EASING.emphasis,
    },
};

export const staggerContainer = {
    animate: {
        transition: {
            staggerChildren: 0.06,
            delayChildren: 0.04,
        },
    },
};

export const staggerItem: AnimationPreset = {
    initial: { opacity: 0, y: DISTANCE.small },
    animate: { opacity: 1, y: 0 },
    transition: {
        duration: DURATION.fast,
        ease: EASING.enter,
    },
};
