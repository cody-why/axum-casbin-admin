use futures::executor::block_on;
use rbatis::RBatis;
use std::sync::OnceLock;

use crate::config::Config;
use crate::utils::cache::cache_service::CacheService;
use crate::utils::db::init_db;

pub mod sys_menu_service;
pub mod sys_role_service;
pub mod sys_trash_service;
pub mod sys_user_service;

pub mod casbin_service;
pub mod login_service;

// pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(||{
//     ServiceContext::new()
// });

pub fn context() -> &'static ServiceContext {
    static CONTEXT: OnceLock<ServiceContext> = OnceLock::new();
    CONTEXT.get_or_init(|| {
        let fut = async {
            let s = ServiceContext::new();
            s.init().await;
            s
        };
        block_on(fut)
    })
}

#[derive(Default)]
pub struct ServiceContext {
    pub config: Config,
    pub rb: RBatis,
    pub cache_service: CacheService,
}

impl ServiceContext {
    pub fn new() -> Self {
        let config = Config::new();
        let cache_service = CacheService::build(&config.cache_type).unwrap();

        Self {
            config,
            rb: RBatis::new(),
            cache_service,
        }
    }

    /// must call this method before using any service
    async fn init(&self) -> &Self {
        init_db(&self.config, &self.rb).await;
        self
    }
}
