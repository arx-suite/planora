import { components } from "@planora/api-types";

declare global {
    // user profile
    export type UserProfile = components["schemas"]["UserProfile"];

    // workspace
    export type Organization = components["schemas"]["OrganizationRow"];
    export type OrganizationResource = components["schemas"]["OrganizationResourceRow"];
    export type OrganizationFeature = components["schemas"]["OrganizationFeatureRow"];

    // TODO: replace these types
    export interface Workspace {
        info: Organization;
        features: string[];
        projects?: Project[];
        spaces?: Space[];
    }

    export interface Space {
        name: string;
        description: string;
        role: string;
        projects: Project[];
    }

    export interface Project {
        projectId: string;
        name: string;
        description: string;
        role: string;
        labels: string[];
    }

    export type Task = {
        projectId: string;
        meta: TaskMeta;
        subtasks: SubTask[];
        comments: TaskComment[];
    };

    export enum TaskStatus {
        Backlog = "backlog", // Not started, waiting
        Planned = "planned", // Not started, waiting
        InProgress = "in-progress", // Actively working
        InReview = "in-review", // Actively working
        Blocked = "blocked", // Can't move forward
        Done = "done", // Completed
        Archived = "archived", // No longer relevant
    }

    export enum TaskPriority {
        Low = "low",
        Medium = "medium",
        High = "high",
        Critical = "critical",
    }

    // TODO: this might be removed from the future
    //       it depends on the usecase of this
    export enum TaskStrategy {
        Do = "do",
        Schedule = "schedule",
        Delegate = "delegate",
        Eliminate = "eliminate",
    }

    // general task types
    export enum TaskType {
        General = "general",
        Feature = "feature",
        Bug = "bug",
        Improvement = "improvement",
        Research = "research",
        Chore = "chore",
        Documentation = "documentation",
        Decision = "decision",
    }

    export type TaskMeta = {
        taskId: string;
        taskName: string;
        description?: string;
        type: TaskType;
        assignor?: string;
        assignee?: string;

        status: TaskStatus;
        priority: TaskPriority;
        tags: string[];

        // task dates
        startDate?: Date;
        dueDate?: Date;
        completedAt?: Date;

        // time tracking
        estimatedHours?: number;
        actualHours: number;
        progress: number;

        createdAt: Date;
        updatedAt: Date;
    };

    export type SubTask = {
        subTaskId: string;
        name: string;
        description: string;
        startedAt?: string;
        completedAt?: string;
        createdAt: string;
    };

    export type TaskComment = {
        commendId: string;
        taskId: string;
        author: string;
        content: string;
        createdAt: string;
    };
}
