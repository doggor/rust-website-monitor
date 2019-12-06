use ssl_expiration::SslExpiration;
use chrono::{Utc, Duration};
use crate::services::scheduler::{Task, TaskResult};
use crate::services::storage::Storage;

pub struct HttpsCertTask {
    storage: Storage,
}

impl HttpsCertTask {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

impl Task for HttpsCertTask {
    fn run(&self) -> TaskResult {
        if let Ok(sites) = self.storage.list_sites() {
            for site in sites {
                match SslExpiration::from_domain_name(&site.domain) {
                    Ok(expiration) => {
                        let expiration_secs = expiration.secs();
                        let dt = Utc::now() + Duration::seconds(expiration_secs.into());
                        let dt = dt.format("%Y-%m-%d %H:%M:%S").to_string();
                        let updated = self.storage.update_site(site.id, None, Some(&dt[..]), None);
                        if let Err(err) = updated {
                            println!("Fail to update expiry date of site {}: {:?}", site.domain, err);
                        }
                        println!("{} expired at {} {}", site.domain, dt, expiration_secs);
                    },
                    Err(err) => {
                        println!("{:?}", err);
                    }
                }
            }
        }
        TaskResult::Continue
    }
}
