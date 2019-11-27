mod site_storage;

use r2d2;
use r2d2_sqlite::SqliteConnectionManager;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Storage {
    pool: Arc<r2d2::Pool<SqliteConnectionManager>>,
}

impl Storage {
    pub fn new(path: &str) -> Self {
        let manager = SqliteConnectionManager::file(path);
        let pool = r2d2::Pool::new(manager).unwrap();

        let storage = Self{ pool: Arc::new(pool) };

        //init
        let init_result = storage.init_site_storage();

        if let Err(err) = init_result {
            panic!(format!("Fail to initialize storage: {}", err.description()));
        }

        storage
    }
}