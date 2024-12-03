use crate::{config::Config, service::sys_trash_service::SysTrashService, Error};
use rbatis::{
    rbdc::{self, pool::conn_manager::ConnManager},
    RBatis,
};
use rbdc_pool_deadpool::DeadPool;
use std::{sync::Arc, time::Duration};
use tracing::info;

/// erverwhere use the `pool!` macro to get a reference to the `RBatis` pool.
#[macro_export]
macro_rules! pool {
    () => {
        &$crate::service::context().rb
    };
}

pub fn get_pool() -> &'static RBatis {
    &crate::service::context().rb
}

/// init database pool
pub async fn init_db(config: &Config, rb: &RBatis) {
    info!("rbatis pool init ({:?})...", config.db.url.split('@').nth(1));
    // let mut rb = RBatis::new();
    // rb.init(rbdc_mysql::Driver {}, config.db.url.as_str())
    //     .expect("rbatis pool init fail!");

    init_option::<_, DeadPool>(rbdc_mysql::Driver {}, config.db.url.as_str(), rb).unwrap();

    rb.intercepts.push(Arc::new(SysTrashService::new()));

    let pool = rb.get_pool().unwrap();

    pool.set_max_open_conns(config.db.max_connections as u64)
        .await;
    pool.set_max_idle_conns(config.db.min_connections as u64)
        .await;
    pool.set_timeout(Some(Duration::from_secs(config.db.connect_timeout as u64)))
        .await;

    // test conn
    pool.get().await.expect("get conn fail!");
    let state = pool.state().await;
    info!("Rbatis pool init success! pool state = {}", state);
}

fn init_option<Driver, Pool>(driver: Driver, url: &str, rb: &RBatis) -> Result<(), Error>
where
    Driver: rbdc::db::Driver + 'static,
    Pool: rbdc::pool::Pool + 'static,
{
    let pool = Pool::new(ConnManager::new(driver, url)?)?;
    rb.init_pool(pool)?;
    Ok(())
}
