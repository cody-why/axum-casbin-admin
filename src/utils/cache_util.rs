/// CechedStruct need impl Clone.\
/// impl_cache!(Struct=>CechedStruct, idl_secs, live_secs);\
/// idl_secs: when read cache, then live time is idl_secs.\
/// live_secs: when set cache, then live time is live_secs.
///
/// # Examples
/// ``` no_run
/// // impl SysUser, cache SysUser
/// impl_cache!(SysUser);
/// // impl SysRole, 30 seconds expire time
/// impl_cache!(SysRole, 30);
/// // impl MyCache, cache SysUser
/// struct MyCache {}
/// impl_cache!(MyCache=>Arc<SysUser>, 30*60);
/// ```
#[macro_export]
macro_rules! impl_cache {
    // default cache type is Vec<T>, no expired
    ($table:ty) => {
        $crate::impl_cache!($table=>Vec<$table>, 0);
    };
    // default cache type is Vec<T>, idl_secs, when read cache, then live time keep idl_secs
    ($table:ty, $idl_secs:expr) => {
        $crate::impl_cache!($table=>Vec<$table>, $idl_secs, 0);
    };
    // default cache type is Vec<T>, set idl_secs and live_secs
    ($table:ty, $idl_secs:expr,$live_secs:expr) => {
        $crate::impl_cache!($table=>Vec<$table>, $idl_secs, live_secs);
    };
    // cache type is container, no expired
    ($table:ty=>$container:path) => {
        $crate::impl_cache!($table=>$container, 0, 0);
    };
    // cache type is container, idl_secs, when read cache, then live time is idl_secs
    ($table:ty=>$container:path, $idl_secs:expr) => {
        $crate::impl_cache!($table=>$container, $idl_secs, 0);
    };
    // cache type is container, set idl_secs and live_secs
    ($table:ty=>$container:path, $idl_secs:expr, $live_secs:expr) => {
        #[allow(unused)]
        impl $table {
            // const CACHEKEY: &'static str = "ckey";
            /// cache instance
            pub fn cache() -> &'static mini_moka::sync::Cache<String, $container> {
                use std::{sync::OnceLock, time::Duration};
                pub use mini_moka::sync::Cache;
                static CACHE: OnceLock<Cache<String, $container>> = OnceLock::new();
                CACHE.get_or_init(|| {
                    if $live_secs > 0 {
                        if $idl_secs == 0 {
                            Cache::builder().time_to_live(Duration::from_secs($live_secs)).build()
                        } else {
                            Cache::builder().time_to_live(Duration::from_secs($live_secs))
                            .time_to_idle(Duration::from_secs($idl_secs))
                            .build()
                        }
                    }else{
                        if $idl_secs == 0 {
                            Cache::builder().build()
                        } else {
                            Cache::builder().time_to_idle(Duration::from_secs($idl_secs)).build()
                        }
                    }
                })
            }

            /// get cache data by key
            pub fn get_cache(key: impl Into<String>) -> Option<$container>{
                Self::cache().get(&key.into())
            }
            /// set cache data by key
            pub fn set_cache(key: impl Into<String>, data: $container) {
                Self::cache().insert(key.into(), data);
            }
            /// remove cache data by key
            pub fn remove_cache(key: impl Into<String>) {
                Self::cache().invalidate(&key.into());
            }
            /// remove all cache data
            pub fn remove_all_cache() {
                Self::cache().invalidate_all();
            }

        }
    };

}

/// select from db, cache it, else return cache
///
/// must impl_cache first
///
#[macro_export]
macro_rules! impl_cache_db {
    ($table:ty) => {
        impl $table {
            /// select all from db，cache it
            pub async fn select_all_cache(rb: &rbatis::RBatis) -> rbatis::Result<Vec<$table>> {
                if let Some(v) = Self::get_cache("select_all") {
                    return Ok(v);
                }
                // let rb = crate::pool!();
                let result = Self::select_all(rb).await?;
                Self::set_cache("select_all", result.clone());
                Ok(result)
            }

            pub fn remove_select_all_cache() {
                Self::remove_cache("select_all");
            }

            /// select by id，cache it
            pub async fn select_by_id_cache<T>(rb: &rbatis::RBatis, id: T) -> rbatis::Result<Vec<$table>>
            where
                T: std::fmt::Display + serde::Serialize,
            {
                Self::select_by_column_cache(rb, "id", id).await
            }

            pub fn remove_by_id_cache<T>(id: T)
            where
                T: std::fmt::Display,
            {
                let key = format!("id_{}", id);
                Self::remove_cache(&key);
            }

            /// select by column，cache it
            pub async fn select_by_column_cache<T>(rb: &rbatis::RBatis, column: &str, val: T) -> rbatis::Result<Vec<$table>>
            where
                T: std::fmt::Display + serde::Serialize,
            {
                let key = format!("{}_{}", column, val);
                if let Some(v) = Self::get_cache(&key) {
                    return Ok(v);
                }
                // let rb = crate::pool!();
                let result = Self::select_by_column(rb, column, val).await?;
                if result.len() > 0 {
                    Self::set_cache(&key, result.clone());
                }
                Ok(result)
            }

            pub fn remove_by_column_cache<T>(column: &str, val: T)
            where
                T: std::fmt::Display,
            {
                let key = format!("{}_{}", column, val);
                Self::remove_cache(&key);
            }
        }
    };
}
