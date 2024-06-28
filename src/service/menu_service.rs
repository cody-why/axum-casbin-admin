use crate::model::menu::SysMenu;
use crate::pool;
use crate::vo::menu_vo::*;
use crate::Error;
use crate::Result;

// #[cached(time = 300, result = true)]
pub async fn menu_list(item: MenuListReq) -> Result<Vec<MenuListData>> {
    let rb = pool!();
    let result = SysMenu::select_all_cache(rb).await?;
    // 使用闭包来定义过滤条件
    let status = item.status.map(|x| x.parse::<i32>().unwrap_or(0));
    let filter_condition = |x: &SysMenu| -> bool {
        match &status {
            Some(s) => x.status == *s,
            None => true,
        }
    };

    let menu_list: Vec<MenuListData> = result.into_iter().filter(filter_condition).map(MenuListData::from).collect();

    Ok(menu_list)
}

// 添加菜单
pub async fn menu_save(item: MenuSaveReq) -> Result<u64> {
    let rb = pool!();

    let sys_menu = SysMenu::from(item);

    let result = SysMenu::insert(rb, &sys_menu).await?;
    SysMenu::remove_cached();
    Ok(result.rows_affected)
}

// 更新菜单
pub async fn menu_update(item: MenuUpdateReq) -> Result<u64> {
    let rb = pool!();
    let sys_menu = SysMenu::from(item);
    let result = SysMenu::update_by_column(rb, &sys_menu, "id").await?;
    SysMenu::remove_cached();
    Ok(result.rows_affected)
}

// 删除菜单信息
pub async fn menu_delete(item: MenuDeleteReq) -> Result<u64> {
    let rb = pool!();
    let mut count = 0;
    for id in item.ids {
        //有下级的时候 不能直接删除
        let menus = SysMenu::select_by_column(rb, "parent_id", &id).await?;
        if !menus.is_empty() {
            if count > 0 {
                SysMenu::remove_cached();
            }
            return Error::err("有下级菜单,不能删除");
        }
        let result = SysMenu::delete_by_column(rb, "id", &id).await?;
        count += result.rows_affected;
    }
    SysMenu::remove_cached();
    Ok(count)
}
