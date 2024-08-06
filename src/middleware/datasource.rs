use std::time::Duration;

use lazy_static::lazy_static;
use rbatis::RBatis;
use rbdc_pg::PgDriver;

use crate::config::CONFIG;

lazy_static! {
    pub static ref DB: DataSource = DataSource::default();
}

pub struct DataSource {
    pub rb: RBatis,
}

impl Default for DataSource {
    fn default() -> Self {
        let rb = RBatis::new();
        DataSource { rb }
    }
}

impl DataSource {
    pub async fn init_database(&self) {
        log::info!("rbatis pool init ({})...", CONFIG.database.url);
        //include auto choose driver struct by 'config.db_url'
        self.rb.init(PgDriver {}, &CONFIG.database.url)
            .expect("rbatis pool init fail!");
        let pool = self.rb.get_pool().unwrap();
        //max connections
        pool.set_max_open_conns(CONFIG.database.pool_size as u64).await;
        //max timeout
        pool.set_timeout(Some(Duration::from_secs(CONFIG.database.pool_timeout as u64))).await;
        log::info!("rbatis pool init success! pool state = {}",
            self.rb.get_pool().expect("pool not init!").state().await
        );
    }
}
