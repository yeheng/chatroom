use std::time::Duration;

use rbatis::RBatis;
use rbdc_pg::PgDriver;

use crate::config::config::ApplicationConfig;

pub async fn init_database(config: &ApplicationConfig) -> RBatis {
    let rb = RBatis::new();
    log::info!("rbatis pool init ({})...", config.database.url);
    //include auto choose driver struct by 'config.db_url'
    rb.link(PgDriver {}, &config.database.url)
        .await
        .expect("rbatis pool init fail!");
    let pool = rb.get_pool().unwrap();
    //max connections
    pool.set_max_open_conns(config.database.pool_size as u64).await;
    //max timeout
    pool.set_timeout(Some(Duration::from_secs(config.database.pool_timeout as u64))).await;
    log::info!("rbatis pool init success! pool state = {}",
        rb.get_pool().expect("pool not init!").state().await
    );

    rb
}
