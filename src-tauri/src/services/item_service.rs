use crate::db::Db;
use crate::dto::item::CreateItemRequest;
use crate::error::{AppError, AppResult};
use crate::models::item::{Item, ItemStatus};
use crate::repositories::item_repo::ItemRepository;
use crate::utils::{id::new_id, time::now_ms};

const MAX_TITLE_LEN: usize = 500;

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
        let title = request.title.trim();
        if title.is_empty() {
            return Err(AppError::Validation("Title must not be empty".into()));
        }
        if title.chars().count() > MAX_TITLE_LEN {
            return Err(AppError::Validation(format!(
                "Title must be at most {MAX_TITLE_LEN} characters"
            )));
        }

        let now = now_ms();
        let item = Item {
            id: new_id(),
            title: title.to_string(),
            status: ItemStatus::Open,
            created_at: now,
            updated_at: now,
        };

        let conn = self.db.lock()?;
        ItemRepository::insert(&conn, &item)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn service() -> ItemService {
        ItemService::new(Db::open_in_memory().unwrap())
    }

    fn request(title: &str) -> CreateItemRequest {
        CreateItemRequest {
            title: title.into(),
        }
    }

    #[test]
    fn create_trims_title_and_starts_open() {
        let item = service().create(request("  call the bank  ")).unwrap();
        assert_eq!(item.title, "call the bank");
        assert_eq!(item.status, ItemStatus::Open);
    }

    #[test]
    fn create_rejects_blank_title() {
        let err = service().create(request("   ")).unwrap_err();
        assert!(matches!(err, AppError::Validation(_)));
    }

    #[test]
    fn list_returns_newest_first() {
        let service = service();
        service.create(request("first")).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(2));
        service.create(request("second")).unwrap();

        let titles: Vec<_> = service.list().unwrap().into_iter().map(|i| i.title).collect();
        assert_eq!(titles, ["second", "first"]);
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
        assert!(matches!(service.delete(&item.id), Err(AppError::NotFound(_))));
    }
}
