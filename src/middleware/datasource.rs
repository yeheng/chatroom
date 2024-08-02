use std::time::Duration;

use once_cell::sync::Lazy;
use rbatis::RBatis;
use rbdc_pg::PgDriver;

use crate::config::config::CONFIG;

pub static DATA_SOURCE: Lazy<DataSource> = Lazy::new(DataSource::default);

pub struct DataSource {
    pub rb: RBatis,
}
impl Default for DataSource {
    fn default() -> Self {
        Self {
            rb: RBatis::new(),
        }
    }
}

pub async fn init_database() {
    log::info!("rbatis pool init ({})...", CONFIG.database.url);
    //include auto choose driver struct by 'config.db_url'
    DATA_SOURCE.rb.link(PgDriver {}, &CONFIG.database.url)
        .await
        .expect("rbatis pool init fail!");
    let pool = DATA_SOURCE.rb.get_pool().unwrap();
    //max connections
    pool.set_max_open_conns(CONFIG.database.pool_size as u64).await;
    //max timeout
    pool.set_timeout(Some(Duration::from_secs(CONFIG.database.pool_timeout as u64))).await;
    log::info!("rbatis pool init success! pool state = {}",
        DATA_SOURCE.rb.get_pool().expect("pool not init!").state().await
    );
}

#[macro_export]
macro_rules! pool {
    () => {
        &DATA_SOURCE.rb
    };
}
