use casbin::{CoreApi, Enforcer};
use casbin_rb_adapter::RbatisAdapter;
// use parking_lot::RwLock;

use tokio::sync::{OnceCell, RwLock};

use crate::pool;
/// 获取casbin权限管理器的写锁
#[macro_export]
macro_rules! casbin_write {
    () => {
        $crate::utils::casbin::enforcer().await.write().await
    };
}

/// 获取casbin权限管理器的读锁
#[macro_export]
macro_rules! casbin_read {
    () => {
        $crate::utils::casbin::enforcer().await.read().await
    };
}

/// 获取casbin权限管理器
pub async fn enforcer() -> &'static RwLock<Enforcer> {
    static ENFORCER: OnceCell<RwLock<Enforcer>> = OnceCell::const_new();
    ENFORCER
        .get_or_init(|| async {
            let rb = pool!();
            let casbin = RbatisAdapter::new(rb).await.unwrap();
            let e = Enforcer::new("config/auth_model.conf", casbin)
                .await
                .unwrap();
            println!("casbin init success");
            RwLock::new(e)
        })
        .await
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use casbin_rb_adapter::casbin::{MgmtApi, RbacApi};

    use super::*;

    #[ignore]
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn enforcer_test() {
        let mut e = casbin_write!();
        // 角色id不能和用户id相同
        let roles = e.get_roles_for_user("1", None);
        println!("用户的角色: {:?}", roles); // ["r1"]

        let pms = e.get_implicit_permissions_for_user("1", None);
        println!("用户的权限: {:?}", pms); // [["r1", "*", "*"]]

        let users = e.get_users_for_role("r1", None);
        println!("角色的用户: {:?}", users); // ["1"]

        let roles = e.get_implicit_permissions_for_user("r1", None);
        println!("角色的权限: {:?}", roles); // [["1", "*", "*"]]

        let pms = e.get_all_policy();
        println!("所有策略: {:?}", pms); // [["p", "p", "r1", "*", "*"]]

        drop(e);

        let e = casbin_read!();

        let ok = e.enforce(("1", "/anyhow", "write"));
        println!("Match is {:?}", ok);
    }
}
