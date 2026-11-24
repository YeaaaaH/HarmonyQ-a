use std::path::Path;

use crate::db::Db;
use crate::error::AppResult;
use crate::services::item_service::ItemService;
use crate::services::project_service::ProjectService;

pub struct AppState {
    pub items: ItemService,
    pub projects: ProjectService,
}

impl AppState {
    pub fn init(data_dir: &Path) -> AppResult<Self> {
        std::fs::create_dir_all(data_dir)?;
        let db = Db::open(&data_dir.join("harmonyq-a.db"))?;
        Ok(Self {
            items: ItemService::new(db.clone()),
            projects: ProjectService::new(db),
        })
    }
}
