use std::path::Path;

use crate::db::Db;
use crate::error::AppResult;
use crate::services::item_service::ItemService;

pub struct AppState {
    pub items: ItemService,
}

impl AppState {
    pub fn init(data_dir: &Path) -> AppResult<Self> {
        std::fs::create_dir_all(data_dir)?;
        let db = Db::open(&data_dir.join("harmonyq-a.db"))?;
        Ok(Self {
            items: ItemService::new(db),
        })
    }
}
