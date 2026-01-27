export const TASK_VIEWS = ["list", "board", "timeline"] as const;
export type TaskViewsList = (typeof TASK_VIEWS)[number];
