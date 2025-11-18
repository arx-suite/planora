-- Add up migration script here

/* === types === */
CREATE TYPE task_status AS ENUM ('not_started', 'active', 'in_review', 'blocked', 'archived', 'done');
CREATE TYPE task_priority AS ENUM ('eliminate', 'delegate', 'schedule', 'low', 'medium', 'high', 'critical');


/* === tables === */
CREATE TABLE IF NOT EXISTS tasks (
    task_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(project_id) ON DELETE CASCADE,
    parent_id UUID REFERENCES tasks(task_id) ON DELETE CASCADE,

    task_name VARCHAR(255) NOT NULL,
    description TEXT,
    type VARCHAR(255) DEFAULT 'general',
    
    assignor UUID NOT NULL REFERENCES project_members(member_id),
    assignee UUID NOT NULL REFERENCES project_members(member_id),

    status task_status NOT NULL DEFAULT 'not started',
    priority task_priority NOT NULL DEFAULT 'medium',

    estimated_hours NUMERIC(6,2),
    actual_hours NUMERIC(6,2) DEFAULT 0,

    start_date TIMESTAMPTZ,
    due_date TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,

    progress SMALLINT DEFAULT 0 CHECK (progress BETWEEN 0 AND 100),
    tags TEXT[] DEFAULT '{}',

    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),

    UNIQUE (project_id, task_name),
    CHECK (project_id <> parent_id)
);

CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id UUID NOT NULL REFERENCES tasks(task_id) ON DELETE CASCADE,
    depends_on UUID NOT NULL REFERENCES tasks(task_id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, depends_on),
    CHECK (task_id <> depends_on)
);

CREATE TABLE IF NOT EXISTS task_comments (
    comment_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID NOT NULL REFERENCES tasks(task_id) ON DELETE CASCADE,
    author UUID NOT NULL REFERENCES project_members(member_id),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE IF NOT EXISTS task_activity_log (
    log_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID NOT NULL REFERENCES tasks(task_id) ON DELETE CASCADE,
    actor UUID NOT NULL REFERENCES project_members(member_id),
    action VARCHAR(100) NOT NULL,
    old_value JSONB,
    new_value JSONB,
    created_at TIMESTAMPTZ DEFAULT now()
);


/* === indexes === */
CREATE INDEX IF NOT EXISTS idx_project_tasks ON tasks(project_id);
CREATE INDEX IF NOT EXISTS idx_task_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_task_assignee ON tasks(assignee);
CREATE INDEX IF NOT EXISTS idx_task_due_date ON tasks(due_date);
CREATE INDEX idx_tasks_tags ON tasks USING gin (tags);


/* === functions / triggers === */
CREATE OR REPLACE FUNCTION update_task_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_update_timestamp
BEFORE UPDATE ON tasks
FOR EACH ROW
EXECUTE FUNCTION update_task_timestamp();

CREATE OR REPLACE FUNCTION recalc_project_progress(project UUID)
RETURNS VOID AS $$
DECLARE
    total_tasks INTEGER;
    completed_tasks INTEGER;
    progress NUMERIC;
BEGIN
    SELECT COUNT(*) INTO total_tasks FROM tasks WHERE project_id = project;
    SELECT COUNT(*) INTO completed_tasks FROM tasks WHERE project_id = project AND status = 'done';
  
    IF total_tasks > 0 THEN
        progress := (completed_tasks::NUMERIC / total_tasks) * 100;
        UPDATE projects SET progress_percent = progress WHERE project_id = project;
    END IF;
END;
$$ LANGUAGE plpgsql;
