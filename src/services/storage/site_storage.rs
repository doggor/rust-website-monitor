use std::error::Error;
use serde::{Serialize, Deserialize};
use rusqlite::params;
use rusqlite::ToSql;
use chrono::Utc;
use super::Storage;

#[derive(Serialize, Deserialize, Debug)]
pub struct SiteRow {
    pub id: u32,
    pub domain: String,
    pub cert_expired_at: String,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl Storage {
    pub fn init_site_storage(&self) -> Result<(), Box<dyn Error>> {
        let conn = self.pool.get()?;

        conn.execute_batch(
            "
            BEGIN;
            CREATE TABLE IF NOT EXISTS sites (
                id              INTEGER PRIMARY KEY,
                domain          TEXT NOT NULL DEFAULT '',
                cert_expired_at TEXT NOT NULL DEFAULT '',
                active          INTEGER DEFAULT 1,
                created_at      TEXT NOT NULL DEFAULT '',
                updated_at      TEXT NOT NULL DEFAULT '',
                deleted_at      TEXT NOT NULL DEFAULT ''
            );
            CREATE INDEX IF NOT EXISTS sites_deleted_at_idx ON sites (deleted_at);
            COMMIT;
            "
        )?;

        Ok(())
    }

    pub fn list_sites(&self) -> Result<Vec<SiteRow>, Box<dyn Error>> {
        let conn = self.pool.get()?;

        let mut statment = conn.prepare("SELECT id, domain, cert_expired_at, active, created_at, updated_at FROM sites WHERE deleted_at = ''")?;

        let site_iter = statment.query_map(params![], |row| {
            Ok(SiteRow {
                id: row.get(0)?,
                domain: row.get(1)?,
                cert_expired_at: row.get(2)?,
                active: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;

        let mut result = Vec::new();

        for site_result in site_iter {
            if let Ok(site_row) = site_result {
                result.push(site_row);
            } else if let Err(err) = site_result {
                println!("Extract with error: {}", err);
            }
        }

        Ok(result)
    }

    pub fn add_site(&self, domain: &str) -> Result<(), Box<dyn Error>> {
        let now = Self::now_string();

        let conn = self.pool.get()?;

        conn.execute("INSERT INTO sites (domain, created_at, updated_at) VALUES (?, ?, ?)", params![domain, now, now])?;

        Ok(())
    }

    pub fn update_site(&self, id: u32, domain: Option<&str>, cert_expired_at: Option<&str>, active: Option<bool>) -> Result<(), Box<dyn Error>> {
        let conn = self.pool.get()?;

        let mut query = String::from("UPDATE sites SET updated_at = ?");
        let mut parameters: Vec<Box<dyn ToSql>> = vec![ Box::new(Self::now_string()) ];

        if let Some(domain) = domain {
            query = format!("{}, domain = ?", query);
            parameters.push(Box::new(String::from(domain)));
        }

        if let Some(cert_expired_at) = cert_expired_at {
            query = format!("{}, cert_expired_at = ?", query);
            parameters.push(Box::new(String::from(cert_expired_at)));
        }

        if let Some(active) = active {
            query = format!("{}, active = ?", query);
            parameters.push(Box::new(if active { "1" } else { "0" }));
        }

        query = format!("{} WHERE id = ?", query);
        parameters.push(Box::new(id.to_string()));

        conn.execute(&query, parameters)?;

        Ok(())
    }

    pub fn remove_site(&self, id: u32) -> Result<(), Box<dyn Error>> {
        let conn = self.pool.get()?;

        conn.execute("UPDATE sites SET deleted_at = ? WHERE id = ?", params![Self::now_string(), id.to_string()])?;

        Ok(())
    }

    fn now_string() -> String {
        Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }
}