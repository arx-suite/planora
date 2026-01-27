import { useState } from "react";
import { TaskBoardView } from "../task-views/board";
import { TaskCommandBar } from "./command-bar";
import type { TaskViewsList } from "./types";

export function TaskSection() {
    const [view, setView] = useState<TaskViewsList>("board");

    return (
        <>
            <TaskCommandBar view={view} setView={setView} />
            {view === "board" ? (
                <TaskBoardView />
            ) : view === "list" ? (
                <div>Task List View</div>
            ) : view === "timeline" ? (
                <div>Task Timeline</div>
            ) : (
                ""
            )}
        </>
    );
}
