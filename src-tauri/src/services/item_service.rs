use std::cmp::Ordering;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::db::Db;
use crate::dto::item::{CreateItemRequest, UpdateItemRequest};
use crate::error::{AppError, AppResult};
use crate::models::item::{Item, ItemStatus, Priority, TodayView};
use crate::models::project::ProjectStatus;
use crate::repositories::item_repo::ItemRepository;
use crate::services::project_service;
use crate::utils::{id::new_id, time::now_ms, validate};

const MAX_TITLE_LEN: usize = 500;
const MAX_NOTE_LEN: usize = 5000;

pub struct ItemService {
    db: Db,
}

impl ItemService {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub fn list(&self) -> AppResult<Vec<Item>> {
        let conn = self.db.lock()?;
        ItemRepository::list(&conn)
    }

    /// `today` is passed in rather than read from the clock so the rules are testable.
    pub fn today(&self, today: NaiveDate) -> AppResult<TodayView> {
        let date = today.format("%Y-%m-%d").to_string();
        let conn = self.db.lock()?;
        let (mut overdue, mut today): (Vec<_>, Vec<_>) = ItemRepository::list_due_by(&conn, &date)?
            .into_iter()
            .partition(|item| item.due_date.as_deref() != Some(date.as_str()));

        overdue.sort_by(day_order);
        today.sort_by(day_order);
        Ok(TodayView {
            date,
            overdue,
            today,
        })
    }

    /// Returns tasks whose reminder is due at `now` and marks them so each fires once.
    pub fn take_due_reminders(&self, now: NaiveDateTime) -> AppResult<Vec<Item>> {
        let date = now.format("%Y-%m-%d").to_string();
        let time = now.format("%H:%M").to_string();
        let conn = self.db.lock()?;
        let due = ItemRepository::list_pending_reminders(&conn, &date, &time)?;
        let reminded_at = now_ms();
        for item in &due {
            ItemRepository::mark_reminded(&conn, &item.id, reminded_at)?;
        }
        Ok(due)
    }

    pub fn create(&self, request: CreateItemRequest) -> AppResult<Item> {
        let title = validate::required(&request.title, "Title", MAX_TITLE_LEN)?;
        let due_date = validate_date(request.due_date)?;

        let conn = self.db.lock()?;
        ensure_project_accepts(&conn, request.project_id.as_deref(), None)?;
        let now = now_ms();
        let item = Item {
            id: new_id(),
            title,
            status: ItemStatus::Open,
            due_date,
            due_time: None,
            note: String::new(),
            priority: Priority::Normal,
            project_id: request.project_id,
            completed_at: None,
            created_at: now,
            updated_at: now,
        };

        ItemRepository::insert(&conn, &item)?;
        Ok(item)
    }

    pub fn update(&self, id: &str, request: UpdateItemRequest) -> AppResult<Item> {
        let title = validate::required(&request.title, "Title", MAX_TITLE_LEN)?;
        let due_date = validate_date(request.due_date)?;
        let due_time = validate_time(request.due_time)?;
        if due_time.is_some() && due_date.is_none() {
            return Err(AppError::Validation("A time needs a date".into()));
        }
        let note = validate::text(&request.note, "Note", MAX_NOTE_LEN)?;

        let conn = self.db.lock()?;
        let mut item = find(&conn, id)?;
        ensure_project_accepts(
            &conn,
            request.project_id.as_deref(),
            item.project_id.as_deref(),
        )?;
        item.title = title;
        item.due_date = due_date;
        item.due_time = due_time;
        item.note = note;
        item.priority = request.priority;
        item.project_id = request.project_id;
        item.updated_at = now_ms();

        ItemRepository::update_details(&conn, &item)?;
        Ok(item)
    }

    /// Moves the task to `today`, keeping everything else.
    pub fn schedule_today(&self, id: &str, today: NaiveDate) -> AppResult<Item> {
        let conn = self.db.lock()?;
        let mut item = find(&conn, id)?;
        item.due_date = Some(today.format("%Y-%m-%d").to_string());
        item.updated_at = now_ms();
        ItemRepository::update_details(&conn, &item)?;
        Ok(item)
    }

    pub fn set_status(&self, id: &str, status: ItemStatus) -> AppResult<Item> {
        let conn = self.db.lock()?;
        let mut item = find(&conn, id)?;
        let now = now_ms();
        item.completed_at = match status {
            ItemStatus::Done => item.completed_at.or(Some(now)),
            ItemStatus::Open => None,
        };
        item.status = status;
        item.updated_at = now;
        ItemRepository::update_status(&conn, &item)?;
        Ok(item)
    }

    pub fn delete(&self, id: &str) -> AppResult<()> {
        let conn = self.db.lock()?;
        if ItemRepository::delete(&conn, id)? == 0 {
            return Err(AppError::NotFound(format!("Item {id}")));
        }
        Ok(())
    }
}

fn find(conn: &rusqlite::Connection, id: &str) -> AppResult<Item> {
    ItemRepository::find(conn, id)?.ok_or_else(|| AppError::NotFound(format!("Item {id}")))
}

/// New tasks go only into active projects; a task may stay in its current one whatever its status.
fn ensure_project_accepts(
    conn: &rusqlite::Connection,
    project_id: Option<&str>,
    current: Option<&str>,
) -> AppResult<()> {
    let Some(project_id) = project_id else {
        return Ok(());
    };
    let project = project_service::find(conn, project_id)?;
    if project.status != ProjectStatus::Active && current != Some(project_id) {
        return Err(AppError::Validation(format!(
            "\"{}\" is not active; tasks can only go into active projects",
            project.name
        )));
    }
    Ok(())
}

fn day_order(a: &Item, b: &Item) -> Ordering {
    type Key<'a> = (bool, Option<&'a str>, (bool, Option<&'a str>, bool, i64));
    fn key(item: &Item) -> Key<'_> {
        (
            item.status == ItemStatus::Done,
            item.due_date.as_deref(),
            item.within_day_key(),
        )
    }
    key(a).cmp(&key(b))
}

/// Accepts `YYYY-MM-DD`; blank means "no date".
fn validate_date(date: Option<String>) -> AppResult<Option<String>> {
    normalize(date, |value| {
        NaiveDate::parse_from_str(value, "%Y-%m-%d")
            .map(|d| d.format("%Y-%m-%d").to_string())
            .map_err(|_| AppError::Validation(format!("Invalid date: {value}")))
    })
}

/// Accepts `HH:MM`; blank means "no time".
fn validate_time(time: Option<String>) -> AppResult<Option<String>> {
    normalize(time, |value| {
        NaiveTime::parse_from_str(value, "%H:%M")
            .map(|t| t.format("%H:%M").to_string())
            .map_err(|_| AppError::Validation(format!("Invalid time: {value}")))
    })
}

fn normalize(
    value: Option<String>,
    parse: impl Fn(&str) -> AppResult<String>,
) -> AppResult<Option<String>> {
    match value.as_deref().map(str::trim) {
        None | Some("") => Ok(None),
        Some(value) => parse(value).map(Some),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn service() -> ItemService {
        ItemService::new(Db::open_in_memory().unwrap())
    }

    fn request(title: &str) -> CreateItemRequest {
        CreateItemRequest {
            title: title.into(),
            due_date: None,
            project_id: None,
        }
    }

    fn details(title: &str) -> UpdateItemRequest {
        UpdateItemRequest {
            title: title.into(),
            due_date: None,
            due_time: None,
            note: String::new(),
            priority: Priority::Normal,
            project_id: None,
        }
    }

    #[test]
    fn create_trims_title_and_starts_open() {
        let item = service().create(request("  call the bank  ")).unwrap();
        assert_eq!(item.title, "call the bank");
        assert_eq!(item.status, ItemStatus::Open);
        assert_eq!(item.priority, Priority::Normal);
        assert_eq!(item.due_date, None);
    }

    #[test]
    fn create_rejects_blank_title() {
        let err = service().create(request("   ")).unwrap_err();
        assert!(matches!(err, AppError::Validation(_)));
    }

    #[test]
    fn create_accepts_a_due_date() {
        let item = service()
            .create(CreateItemRequest {
                title: "pay rent".into(),
                due_date: Some("2026-11-24".into()),
                project_id: None,
            })
            .unwrap();
        assert_eq!(item.due_date.as_deref(), Some("2026-11-24"));
    }

    #[test]
    fn list_returns_newest_first() {
        let service = service();
        service.create(request("first")).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(2));
        service.create(request("second")).unwrap();

        let titles: Vec<_> = service
            .list()
            .unwrap()
            .into_iter()
            .map(|i| i.title)
            .collect();
        assert_eq!(titles, ["second", "first"]);
    }

    #[test]
    fn update_saves_all_details_and_persists_them() {
        let service = service();
        let item = service.create(request("task")).unwrap();

        let updated = service
            .update(
                &item.id,
                UpdateItemRequest {
                    title: " dentist ".into(),
                    due_date: Some("2026-11-24".into()),
                    due_time: Some("09:30".into()),
                    note: "  bring the card ".into(),
                    priority: Priority::High,
                    project_id: None,
                },
            )
            .unwrap();
        assert_eq!(updated.title, "dentist");
        assert_eq!(updated.note, "bring the card");

        let stored = service.list().unwrap().remove(0);
        assert_eq!(stored.due_date.as_deref(), Some("2026-11-24"));
        assert_eq!(stored.due_time.as_deref(), Some("09:30"));
        assert_eq!(stored.priority, Priority::High);
    }

    #[test]
    fn update_treats_blank_date_and_time_as_none() {
        let service = service();
        let item = service.create(request("task")).unwrap();

        let mut req = details("task");
        req.due_date = Some(" ".into());
        req.due_time = Some("".into());
        let updated = service.update(&item.id, req).unwrap();
        assert_eq!(updated.due_date, None);
        assert_eq!(updated.due_time, None);
    }

    #[test]
    fn update_rejects_invalid_date_time_and_time_without_date() {
        let service = service();
        let item = service.create(request("task")).unwrap();

        let cases = [
            (Some("2026-02-30"), None),
            (Some("24.11.2026"), None),
            (Some("2026-11-24"), Some("25:00")),
            (None, Some("09:30")),
        ];
        for (date, time) in cases {
            let mut req = details("task");
            req.due_date = date.map(Into::into);
            req.due_time = time.map(Into::into);
            let err = service.update(&item.id, req).unwrap_err();
            assert!(matches!(err, AppError::Validation(_)), "{date:?} {time:?}");
        }
    }

    #[test]
    fn update_missing_item_is_not_found() {
        let err = service().update("nope", details("task")).unwrap_err();
        assert!(matches!(err, AppError::NotFound(_)));
    }

    fn day(date: &str) -> NaiveDate {
        NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
    }

    fn planned(
        service: &ItemService,
        title: &str,
        date: Option<&str>,
        time: Option<&str>,
        priority: Priority,
    ) -> Item {
        let item = service.create(request(title)).unwrap();
        service
            .update(
                &item.id,
                UpdateItemRequest {
                    title: title.into(),
                    due_date: date.map(Into::into),
                    due_time: time.map(Into::into),
                    note: String::new(),
                    priority,
                    project_id: None,
                },
            )
            .unwrap()
    }

    fn titles(items: &[Item]) -> Vec<&str> {
        items.iter().map(|i| i.title.as_str()).collect()
    }

    #[test]
    fn today_splits_overdue_and_today_and_skips_the_rest() {
        let service = service();
        planned(
            &service,
            "yesterday",
            Some("2026-11-22"),
            None,
            Priority::Normal,
        );
        planned(
            &service,
            "today",
            Some("2026-11-23"),
            None,
            Priority::Normal,
        );
        planned(
            &service,
            "tomorrow",
            Some("2026-11-24"),
            None,
            Priority::Normal,
        );
        planned(&service, "undated", None, None, Priority::Normal);
        let done_old = planned(
            &service,
            "done long ago",
            Some("2026-11-01"),
            None,
            Priority::Normal,
        );
        service.set_status(&done_old.id, ItemStatus::Done).unwrap();

        let view = service.today(day("2026-11-23")).unwrap();
        assert_eq!(view.date, "2026-11-23");
        assert_eq!(titles(&view.overdue), ["yesterday"]);
        assert_eq!(titles(&view.today), ["today"]);
    }

    #[test]
    fn today_orders_by_time_then_priority_with_done_last() {
        let service = service();
        let date = Some("2026-11-23");
        let done = planned(&service, "done", date, Some("07:00"), Priority::High);
        service.set_status(&done.id, ItemStatus::Done).unwrap();
        planned(&service, "untimed normal", date, None, Priority::Normal);
        planned(&service, "untimed important", date, None, Priority::High);
        planned(&service, "at 14:00", date, Some("14:00"), Priority::Normal);
        planned(&service, "at 09:00", date, Some("09:00"), Priority::Normal);

        let view = service.today(day("2026-11-23")).unwrap();
        assert_eq!(
            titles(&view.today),
            [
                "at 09:00",
                "at 14:00",
                "untimed important",
                "untimed normal",
                "done"
            ]
        );
    }

    #[test]
    fn today_orders_overdue_oldest_first() {
        let service = service();
        planned(
            &service,
            "two days ago",
            Some("2026-11-21"),
            None,
            Priority::Normal,
        );
        planned(
            &service,
            "last week",
            Some("2026-11-16"),
            None,
            Priority::Normal,
        );

        let view = service.today(day("2026-11-23")).unwrap();
        assert_eq!(titles(&view.overdue), ["last week", "two days ago"]);
    }

    #[test]
    fn set_status_updates_the_item() {
        let service = service();
        let item = service.create(request("task")).unwrap();

        let updated = service.set_status(&item.id, ItemStatus::Done).unwrap();
        assert_eq!(updated.status, ItemStatus::Done);
    }

    #[test]
    fn ticking_stamps_completed_at_once_and_unticking_clears_it() {
        let service = service();
        let item = service.create(request("task")).unwrap();
        assert_eq!(item.completed_at, None);

        let done = service.set_status(&item.id, ItemStatus::Done).unwrap();
        let stamped = done.completed_at.expect("completed_at set");
        std::thread::sleep(std::time::Duration::from_millis(2));
        let again = service.set_status(&item.id, ItemStatus::Done).unwrap();
        assert_eq!(again.completed_at, Some(stamped));

        let reopened = service.set_status(&item.id, ItemStatus::Open).unwrap();
        assert_eq!(reopened.completed_at, None);
    }

    #[test]
    fn update_moves_tasks_between_projects_but_not_into_missing_or_inactive_ones() {
        use crate::dto::project::CreateProjectRequest;
        use crate::models::project::ProjectStatus;
        use crate::services::project_service::ProjectService;

        let db = Db::open_in_memory().unwrap();
        let items = ItemService::new(db.clone());
        let projects = ProjectService::new(db);
        let project = projects
            .create(CreateProjectRequest {
                name: "Taxes".into(),
                outcome: String::new(),
            })
            .unwrap();
        let item = items.create(request("file return")).unwrap();
        let into = |project_id: Option<&str>| {
            let mut req = details("file return");
            req.project_id = project_id.map(Into::into);
            req
        };

        let moved = items.update(&item.id, into(Some(&project.id))).unwrap();
        assert_eq!(moved.project_id.as_deref(), Some(project.id.as_str()));
        let out = items.update(&item.id, into(None)).unwrap();
        assert_eq!(out.project_id, None);
        assert!(matches!(
            items.update(&item.id, into(Some("nope"))),
            Err(AppError::NotFound(_))
        ));

        projects
            .set_status(&project.id, ProjectStatus::Paused)
            .unwrap();
        assert!(matches!(
            items.update(&item.id, into(Some(&project.id))),
            Err(AppError::Validation(_))
        ));
        let mut req = request("new step");
        req.project_id = Some(project.id.clone());
        assert!(matches!(items.create(req), Err(AppError::Validation(_))));
    }

    #[test]
    fn a_task_already_in_a_paused_project_can_still_be_edited() {
        use crate::dto::project::CreateProjectRequest;
        use crate::models::project::ProjectStatus;
        use crate::services::project_service::ProjectService;

        let db = Db::open_in_memory().unwrap();
        let items = ItemService::new(db.clone());
        let projects = ProjectService::new(db);
        let project = projects
            .create(CreateProjectRequest {
                name: "Taxes".into(),
                outcome: String::new(),
            })
            .unwrap();
        let mut req = request("file return");
        req.project_id = Some(project.id.clone());
        let item = items.create(req).unwrap();
        projects
            .set_status(&project.id, ProjectStatus::Paused)
            .unwrap();

        let mut edit = details("file the return");
        edit.project_id = Some(project.id.clone());
        assert_eq!(
            items.update(&item.id, edit).unwrap().title,
            "file the return"
        );
    }

    #[test]
    fn schedule_today_changes_only_the_date() {
        let service = service();
        let mut req = details("dentist");
        req.note = "bring the card".into();
        req.priority = Priority::High;
        let item = service.create(request("dentist")).unwrap();
        service.update(&item.id, req).unwrap();

        let moved = service.schedule_today(&item.id, day("2026-11-25")).unwrap();
        assert_eq!(moved.due_date.as_deref(), Some("2026-11-25"));
        assert_eq!(moved.note, "bring the card");
        assert_eq!(moved.priority, Priority::High);
    }

    #[test]
    fn delete_removes_item_and_missing_id_is_not_found() {
        let service = service();
        let item = service.create(request("task")).unwrap();

        service.delete(&item.id).unwrap();
        assert!(service.list().unwrap().is_empty());
        assert!(matches!(
            service.delete(&item.id),
            Err(AppError::NotFound(_))
        ));
    }

    fn at(date: &str, time: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(&format!("{date} {time}"), "%Y-%m-%d %H:%M").unwrap()
    }

    #[test]
    fn reminders_fire_once_for_open_tasks_whose_time_has_come() {
        let service = service();
        planned(
            &service,
            "due",
            Some("2026-11-23"),
            Some("09:00"),
            Priority::Normal,
        );
        planned(
            &service,
            "later",
            Some("2026-11-23"),
            Some("18:00"),
            Priority::Normal,
        );
        planned(
            &service,
            "no time",
            Some("2026-11-23"),
            None,
            Priority::Normal,
        );
        planned(
            &service,
            "yesterday",
            Some("2026-11-22"),
            Some("09:00"),
            Priority::Normal,
        );
        let done = planned(
            &service,
            "done",
            Some("2026-11-23"),
            Some("08:00"),
            Priority::Normal,
        );
        service.set_status(&done.id, ItemStatus::Done).unwrap();

        let fired = service
            .take_due_reminders(at("2026-11-23", "09:00"))
            .unwrap();
        assert_eq!(titles(&fired), ["due"]);
        assert!(service
            .take_due_reminders(at("2026-11-23", "09:01"))
            .unwrap()
            .is_empty());

        let fired = service
            .take_due_reminders(at("2026-11-23", "18:30"))
            .unwrap();
        assert_eq!(titles(&fired), ["later"]);
    }

    #[test]
    fn rescheduling_a_reminded_task_reminds_again() {
        let service = service();
        let item = planned(
            &service,
            "call",
            Some("2026-11-23"),
            Some("09:00"),
            Priority::Normal,
        );
        service
            .take_due_reminders(at("2026-11-23", "09:00"))
            .unwrap();

        planned_update(&service, &item.id, "call (renamed)", "09:00");
        assert!(service
            .take_due_reminders(at("2026-11-23", "10:00"))
            .unwrap()
            .is_empty());

        planned_update(&service, &item.id, "call", "11:00");
        let fired = service
            .take_due_reminders(at("2026-11-23", "11:00"))
            .unwrap();
        assert_eq!(titles(&fired), ["call"]);
    }

    fn planned_update(service: &ItemService, id: &str, title: &str, time: &str) {
        service
            .update(
                id,
                UpdateItemRequest {
                    title: title.into(),
                    due_date: Some("2026-11-23".into()),
                    due_time: Some(time.into()),
                    note: String::new(),
                    priority: Priority::Normal,
                    project_id: None,
                },
            )
            .unwrap();
    }
}
