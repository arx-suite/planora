import type { Variants } from "motion";
import {
    type AnimationPreset,
    pop as popVar,
    slide as slideVar,
    staggerItem as staggerItemVar,
} from "./presets";

export const toVariants = (preset: AnimationPreset): Variants => {
    const motionVariant: Variants = {
        hidden: preset.initial,
        show: {
            ...preset.animate,
            transition: preset.transition,
        },
    };

    if (preset.exit) {
        motionVariant.exit = preset.exit;
    }

    return motionVariant;
};

export const slide = (direction: "up" | "down" | "left" | "right") =>
    toVariants(slideVar(direction));
export const pop = toVariants(popVar);
export const staggerItem = toVariants(staggerItemVar);
export { staggerContainer } from "./presets";
