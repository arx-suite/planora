import { Input, Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@planora/ui";
import { CalendarDays, KanbanSquare, LayoutList, Search } from "lucide-react";
import type React from "react";
import { TASK_VIEWS, type TaskViewsList } from "./types";

const TASK_VIEW_META: Record<TaskViewsList, { label: string; icon: React.ReactNode }> = {
    list: {
        label: "List",
        icon: <LayoutList className="h-4 w-4" />,
    },
    board: {
        label: "Board",
        icon: <KanbanSquare className="h-4 w-4" />,
    },
    timeline: {
        label: "Timeline",
        icon: <CalendarDays className="h-4 w-4" />,
    },
};

type TaskCommandBarProps = {
    view: TaskViewsList;
    setView: React.Dispatch<React.SetStateAction<TaskViewsList>>;
};

export function TaskCommandBar({ view, setView }: TaskCommandBarProps) {
    return (
        <aside className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
            {/* Search */}
            <SearchInput />

            {/* Views */}
            <Select value={view} onValueChange={(v) => setView(v as TaskViewsList)}>
                <SelectTrigger className="w-full sm:w-44">
                    <SelectValue />
                </SelectTrigger>

                <SelectContent>
                    {TASK_VIEWS.map((v) => (
                        <SelectItem key={v} value={v}>
                            <div className="flex items-center gap-2">
                                {TASK_VIEW_META[v].icon}
                                <span>{TASK_VIEW_META[v].label}</span>
                            </div>
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </aside>
    );
}

export function SearchInput() {
    return (
        <div className="relative w-full sm:max-w-sm">
            <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
            <Input type="search" placeholder="Search tasks…" className="pl-9" />
        </div>
    );
}
