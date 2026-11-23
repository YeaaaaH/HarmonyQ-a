use chrono::{NaiveDate, NaiveTime};

use crate::db::Db;
use crate::dto::item::{CreateItemRequest, UpdateItemRequest};
use crate::error::{AppError, AppResult};
use crate::models::item::{Item, ItemStatus, Priority};
use crate::repositories::item_repo::ItemRepository;
use crate::utils::{id::new_id, time::now_ms};

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

    pub fn create(&self, request: CreateItemRequest) -> AppResult<Item> {
        let title = validate_title(&request.title)?;
        let due_date = validate_date(request.due_date)?;

        let now = now_ms();
        let item = Item {
            id: new_id(),
            title,
            status: ItemStatus::Open,
            due_date,
            due_time: None,
            note: String::new(),
            priority: Priority::Normal,
            created_at: now,
            updated_at: now,
        };

        let conn = self.db.lock()?;
        ItemRepository::insert(&conn, &item)?;
        Ok(item)
    }

    pub fn update(&self, id: &str, request: UpdateItemRequest) -> AppResult<Item> {
        let title = validate_title(&request.title)?;
        let due_date = validate_date(request.due_date)?;
        let due_time = validate_time(request.due_time)?;
        if due_time.is_some() && due_date.is_none() {
            return Err(AppError::Validation("A time needs a date".into()));
        }
        let note = request.note.trim().to_string();
        if note.chars().count() > MAX_NOTE_LEN {
            return Err(AppError::Validation(format!(
                "Note must be at most {MAX_NOTE_LEN} characters"
            )));
        }

        let conn = self.db.lock()?;
        let mut item = ItemRepository::find(&conn, id)?
            .ok_or_else(|| AppError::NotFound(format!("Item {id}")))?;
        item.title = title;
        item.due_date = due_date;
        item.due_time = due_time;
        item.note = note;
        item.priority = request.priority;
        item.updated_at = now_ms();

        ItemRepository::update_details(&conn, &item)?;
        Ok(item)
    }

    pub fn set_status(&self, id: &str, status: ItemStatus) -> AppResult<Item> {
        let conn = self.db.lock()?;
        let changed = ItemRepository::update_status(&conn, id, status, now_ms())?;
        if changed == 0 {
            return Err(AppError::NotFound(format!("Item {id}")));
        }
        ItemRepository::find(&conn, id)?.ok_or_else(|| AppError::NotFound(format!("Item {id}")))
    }

    pub fn delete(&self, id: &str) -> AppResult<()> {
        let conn = self.db.lock()?;
        if ItemRepository::delete(&conn, id)? == 0 {
            return Err(AppError::NotFound(format!("Item {id}")));
        }
        Ok(())
    }
}

fn validate_title(title: &str) -> AppResult<String> {
    let title = title.trim();
    if title.is_empty() {
        return Err(AppError::Validation("Title must not be empty".into()));
    }
    if title.chars().count() > MAX_TITLE_LEN {
        return Err(AppError::Validation(format!(
            "Title must be at most {MAX_TITLE_LEN} characters"
        )));
    }
    Ok(title.to_string())
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
        }
    }

    fn details(title: &str) -> UpdateItemRequest {
        UpdateItemRequest {
            title: title.into(),
            due_date: None,
            due_time: None,
            note: String::new(),
            priority: Priority::Normal,
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

    #[test]
    fn set_status_updates_the_item() {
        let service = service();
        let item = service.create(request("task")).unwrap();

        let updated = service.set_status(&item.id, ItemStatus::Done).unwrap();
        assert_eq!(updated.status, ItemStatus::Done);
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
}
