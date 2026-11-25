use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;

use crate::db::Db;
use crate::dto::project::{CreateProjectRequest, UpdateProjectRequest};
use crate::error::{AppError, AppResult};
use crate::models::item::{Item, ItemStatus, Priority};
use crate::models::project::{Project, ProjectStatus, ProjectSummary, TaskTurnedIntoProject};
use crate::repositories::item_repo::ItemRepository;
use crate::repositories::project_repo::ProjectRepository;
use crate::utils::{id::new_id, time::now_ms, validate};

const MAX_NAME_LEN: usize = 200;
const MAX_OUTCOME_LEN: usize = 500;
const MAX_RESUME_NOTE_LEN: usize = 2000;

pub struct ProjectService {
    db: Db,
}

impl ProjectService {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub fn list(&self) -> AppResult<Vec<ProjectSummary>> {
        let conn = self.db.lock()?;
        let mut tasks: HashMap<String, Vec<Item>> = HashMap::new();
        for item in ItemRepository::list_in_projects(&conn)? {
            if let Some(project_id) = item.project_id.clone() {
                tasks.entry(project_id).or_default().push(item);
            }
        }

        let mut summaries: Vec<_> = ProjectRepository::list(&conn)?
            .into_iter()
            .map(|project| {
                let tasks = tasks.remove(&project.id).unwrap_or_default();
                summarize(project, tasks)
            })
            .collect();
        summaries.sort_by(project_order);
        Ok(summaries)
    }

    pub fn get(&self, id: &str) -> AppResult<Project> {
        let conn = self.db.lock()?;
        find(&conn, id)
    }

    pub fn list_tasks(&self, id: &str) -> AppResult<Vec<Item>> {
        let conn = self.db.lock()?;
        find(&conn, id)?;
        let (mut open, mut done): (Vec<_>, Vec<_>) = ItemRepository::list_by_project(&conn, id)?
            .into_iter()
            .partition(|item| item.status == ItemStatus::Open);
        open.sort_by(next_step_order);
        done.sort_by_key(|item| Reverse((item.completed_at, item.updated_at)));
        open.extend(done);
        Ok(open)
    }

    pub fn create(&self, request: CreateProjectRequest) -> AppResult<Project> {
        let name = validate::required(&request.name, "Name", MAX_NAME_LEN)?;
        let outcome = validate::text(&request.outcome, "Outcome", MAX_OUTCOME_LEN)?;

        let project = new_project(name, outcome);
        let conn = self.db.lock()?;
        ProjectRepository::insert(&conn, &project)?;
        Ok(project)
    }

    /// Creates a project named after a loose task and moves the task into it as its first step.
    pub fn create_from_task(&self, item_id: &str) -> AppResult<TaskTurnedIntoProject> {
        let conn = self.db.lock()?;
        let mut task = ItemRepository::find(&conn, item_id)?
            .ok_or_else(|| AppError::NotFound(format!("Item {item_id}")))?;
        if task.project_id.is_some() {
            return Err(AppError::Validation(
                "Only a task without a project can become one".into(),
            ));
        }
        let name: String = task.title.chars().take(MAX_NAME_LEN).collect();
        let project = new_project(name.trim().to_string(), String::new());
        task.project_id = Some(project.id.clone());
        task.updated_at = project.created_at;

        // Both writes or neither: no empty project if moving the task fails.
        let tx = conn.unchecked_transaction()?;
        ProjectRepository::insert(&tx, &project)?;
        ItemRepository::update_project(&tx, item_id, Some(&project.id), task.updated_at)?;
        tx.commit()?;
        Ok(TaskTurnedIntoProject { project, task })
    }

    pub fn update(&self, id: &str, request: UpdateProjectRequest) -> AppResult<Project> {
        let name = validate::required(&request.name, "Name", MAX_NAME_LEN)?;
        let outcome = validate::text(&request.outcome, "Outcome", MAX_OUTCOME_LEN)?;
        let resume_note = validate::text(&request.resume_note, "Resume note", MAX_RESUME_NOTE_LEN)?;

        let conn = self.db.lock()?;
        let mut project = find(&conn, id)?;
        project.name = name;
        project.outcome = outcome;
        project.resume_note = resume_note;
        project.updated_at = now_ms();
        ProjectRepository::update_details(&conn, &project)?;
        Ok(project)
    }

    pub fn set_status(&self, id: &str, status: ProjectStatus) -> AppResult<Project> {
        let conn = self.db.lock()?;
        let mut project = find(&conn, id)?;
        let now = now_ms();
        project.completed_at = match status {
            ProjectStatus::Done => project.completed_at.or(Some(now)),
            ProjectStatus::Active | ProjectStatus::Paused => None,
        };
        project.status = status;
        project.updated_at = now;
        ProjectRepository::update_status(&conn, &project)?;
        Ok(project)
    }

    /// Tasks of a deleted project are kept and lose their project (`ON DELETE SET NULL`).
    pub fn delete(&self, id: &str) -> AppResult<()> {
        let conn = self.db.lock()?;
        if ProjectRepository::delete(&conn, id)? == 0 {
            return Err(AppError::NotFound(format!("Project {id}")));
        }
        Ok(())
    }
}

pub(crate) fn find(conn: &rusqlite::Connection, id: &str) -> AppResult<Project> {
    ProjectRepository::find(conn, id)?.ok_or_else(|| AppError::NotFound(format!("Project {id}")))
}

fn new_project(name: String, outcome: String) -> Project {
    let now = now_ms();
    Project {
        id: new_id(),
        hue: hue_for(&name),
        name,
        outcome,
        status: ProjectStatus::Active,
        resume_note: String::new(),
        created_at: now,
        updated_at: now,
        completed_at: None,
    }
}

fn summarize(project: Project, tasks: Vec<Item>) -> ProjectSummary {
    let last_activity_at = tasks
        .iter()
        .map(|item| item.updated_at)
        .fold(project.updated_at, i64::max);
    let is_open = |item: &Item| item.status == ItemStatus::Open;
    let open_count = tasks.iter().filter(|item| is_open(item)).count();
    let done_count = tasks.len() - open_count;
    let next_step = tasks
        .into_iter()
        .filter(is_open)
        .min_by(next_step_order)
        .filter(|item| item.due_date.is_some() || item.priority == Priority::High);

    ProjectSummary {
        project,
        next_step,
        open_count,
        done_count,
        last_activity_at,
    }
}

/// Unlike the day view, dated steps come before undated ones.
fn next_step_order(a: &Item, b: &Item) -> Ordering {
    type Key<'a> = (bool, Option<&'a str>, (bool, Option<&'a str>, bool, i64));
    fn key(item: &Item) -> Key<'_> {
        (
            item.due_date.is_none(),
            item.due_date.as_deref(),
            item.within_day_key(),
        )
    }
    key(a).cmp(&key(b))
}

fn project_order(a: &ProjectSummary, b: &ProjectSummary) -> Ordering {
    let rank = |s: &ProjectSummary| match s.project.status {
        ProjectStatus::Active => 0,
        ProjectStatus::Paused => 1,
        ProjectStatus::Done => 2,
    };
    rank(a).cmp(&rank(b)).then_with(|| match a.project.status {
        ProjectStatus::Active => match (&a.next_step, &b.next_step) {
            (Some(x), Some(y)) => next_step_order(x, y),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
        .then(b.last_activity_at.cmp(&a.last_activity_at)),
        ProjectStatus::Paused => b.last_activity_at.cmp(&a.last_activity_at),
        ProjectStatus::Done => b.project.completed_at.cmp(&a.project.completed_at),
    })
}

/// Stable per name, spread around the color wheel (FNV-1a hash).
fn hue_for(name: &str) -> u16 {
    let hash = name.to_lowercase().bytes().fold(0x811c_9dc5_u32, |h, b| {
        (h ^ u32::from(b)).wrapping_mul(0x0100_0193)
    });
    (hash % 360) as u16
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::item::{CreateItemRequest, UpdateItemRequest};
    use crate::services::item_service::ItemService;

    fn services() -> (ProjectService, ItemService) {
        let db = Db::open_in_memory().unwrap();
        (ProjectService::new(db.clone()), ItemService::new(db))
    }

    fn project(projects: &ProjectService, name: &str) -> Project {
        projects
            .create(CreateProjectRequest {
                name: name.into(),
                outcome: String::new(),
            })
            .unwrap()
    }

    fn task(items: &ItemService, project: &Project, title: &str, date: Option<&str>) -> Item {
        let item = items
            .create(CreateItemRequest {
                title: title.into(),
                due_date: None,
                project_id: Some(project.id.clone()),
            })
            .unwrap();
        items
            .update(
                &item.id,
                UpdateItemRequest {
                    title: title.into(),
                    due_date: date.map(Into::into),
                    due_time: None,
                    note: String::new(),
                    priority: Priority::Normal,
                    project_id: Some(project.id.clone()),
                },
            )
            .unwrap()
    }

    fn names(summaries: &[ProjectSummary]) -> Vec<&str> {
        summaries.iter().map(|s| s.project.name.as_str()).collect()
    }

    #[test]
    fn create_trims_starts_active_and_keeps_hue_on_rename() {
        let (projects, _) = services();
        let created = project(&projects, "  Move flat  ");
        assert_eq!(created.name, "Move flat");
        assert_eq!(created.status, ProjectStatus::Active);
        assert!(created.hue < 360);

        let renamed = projects
            .update(
                &created.id,
                UpdateProjectRequest {
                    name: "Move to the new flat".into(),
                    outcome: " keys returned ".into(),
                    resume_note: "called the landlord".into(),
                },
            )
            .unwrap();
        assert_eq!(renamed.hue, created.hue);
        assert_eq!(renamed.outcome, "keys returned");
        assert_eq!(
            projects.get(&created.id).unwrap().resume_note,
            "called the landlord"
        );
    }

    #[test]
    fn create_rejects_blank_name() {
        let (projects, _) = services();
        let err = projects
            .create(CreateProjectRequest {
                name: "  ".into(),
                outcome: String::new(),
            })
            .unwrap_err();
        assert!(matches!(err, AppError::Validation(_)));
    }

    #[test]
    fn hue_depends_only_on_the_name() {
        assert_eq!(hue_for("Taxes"), hue_for("taxes"));
        assert_ne!(hue_for("Taxes"), hue_for("Garden"));
    }

    #[test]
    fn summary_has_next_step_and_counts() {
        let (projects, items) = services();
        let p = project(&projects, "Taxes");
        task(&items, &p, "undated", None);
        task(&items, &p, "later", Some("2026-12-10"));
        task(&items, &p, "soon", Some("2026-11-30"));
        let done = task(&items, &p, "done", Some("2026-11-01"));
        items.set_status(&done.id, ItemStatus::Done).unwrap();

        let summary = projects.list().unwrap().remove(0);
        assert_eq!(summary.next_step.unwrap().title, "soon");
        assert_eq!(summary.open_count, 3);
        assert_eq!(summary.done_count, 1);
    }

    #[test]
    fn no_next_step_is_picked_when_no_open_task_has_a_date_or_priority() {
        let (projects, items) = services();
        let p = project(&projects, "Garden");
        task(&items, &p, "plant tulips", None);
        task(&items, &p, "buy soil", None);

        let summary = projects.list().unwrap().remove(0);
        assert!(summary.next_step.is_none());
        assert_eq!(summary.open_count, 2);
    }

    #[test]
    fn list_orders_active_by_next_step_then_paused_then_done() {
        let (projects, items) = services();
        project(&projects, "needs a step");
        let later = project(&projects, "later");
        task(&items, &later, "x", Some("2026-12-20"));
        let sooner = project(&projects, "sooner");
        task(&items, &sooner, "x", Some("2026-11-25"));
        let paused = project(&projects, "paused");
        projects
            .set_status(&paused.id, ProjectStatus::Paused)
            .unwrap();
        let done = project(&projects, "done");
        projects.set_status(&done.id, ProjectStatus::Done).unwrap();

        assert_eq!(
            names(&projects.list().unwrap()),
            ["sooner", "later", "needs a step", "paused", "done"]
        );
    }

    #[test]
    fn status_done_stamps_completion_and_reopening_clears_it() {
        let (projects, _) = services();
        let p = project(&projects, "Taxes");

        let done = projects.set_status(&p.id, ProjectStatus::Done).unwrap();
        assert!(done.completed_at.is_some());
        let reopened = projects.set_status(&p.id, ProjectStatus::Active).unwrap();
        assert_eq!(reopened.completed_at, None);
    }

    #[test]
    fn list_tasks_puts_open_in_next_step_order_then_done() {
        let (projects, items) = services();
        let p = project(&projects, "Taxes");
        task(&items, &p, "undated", None);
        let done = task(&items, &p, "done", None);
        items.set_status(&done.id, ItemStatus::Done).unwrap();
        task(&items, &p, "dated", Some("2026-11-30"));

        let titles: Vec<_> = projects
            .list_tasks(&p.id)
            .unwrap()
            .into_iter()
            .map(|i| i.title)
            .collect();
        assert_eq!(titles, ["dated", "undated", "done"]);
    }

    #[test]
    fn a_task_turned_into_a_project_becomes_its_first_step() {
        let (projects, items) = services();
        let item = items
            .create(CreateItemRequest {
                title: "Renovate the kitchen".into(),
                due_date: None,
                project_id: None,
            })
            .unwrap();

        let TaskTurnedIntoProject { project, task } = projects.create_from_task(&item.id).unwrap();
        assert_eq!(project.name, "Renovate the kitchen");
        assert_eq!(project.status, ProjectStatus::Active);
        assert_eq!(task.project_id.as_deref(), Some(project.id.as_str()));
        assert!(matches!(
            projects.create_from_task(&item.id),
            Err(AppError::Validation(_))
        ));

        let tasks = projects.list_tasks(&project.id).unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, item.id);
        assert!(matches!(
            projects.create_from_task("nope"),
            Err(AppError::NotFound(_))
        ));
        assert_eq!(projects.list().unwrap().len(), 1);
    }

    #[test]
    fn deleting_a_project_keeps_its_tasks() {
        let (projects, items) = services();
        let p = project(&projects, "Taxes");
        let t = task(&items, &p, "file return", None);

        projects.delete(&p.id).unwrap();
        let kept = items.list().unwrap().remove(0);
        assert_eq!(kept.id, t.id);
        assert_eq!(kept.project_id, None);
        assert!(matches!(projects.get(&p.id), Err(AppError::NotFound(_))));
    }
}
